//! C# call-site extraction (field report 2026-11-23, #14).
//!
//! A narrow lexical scan, like [`super::declarations`]: it finds `member(`
//! and `receiver.member(` inside method bodies and records the receiver's
//! declared type when this file declares it (a field, property, parameter or
//! `var x = new T(...)` local). It proves nothing about which implementation
//! runs; resolution against the whole project happens later and stays
//! `inferred_static` or `unresolved`. Comments and string literals are masked
//! first so text inside them is never read as a call.

use std::collections::BTreeMap;

use super::declarations::{CsharpDeclaration, CsharpDeclarationKind};

/// One call site inside a method body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsharpCallSite {
    /// Key of the calling method declaration.
    pub caller_key: String,
    /// Literal member name, e.g. `Execute`.
    pub member: String,
    /// Literal receiver, e.g. `_useCase`, `this` or `Math`; `None` for a bare
    /// call. `Some("?")` when the receiver is an expression (a chained call).
    pub receiver: Option<String>,
    /// Receiver type this file declares for it, generics and `?` stripped.
    pub receiver_type: Option<String>,
    /// One-based source line.
    pub line: usize,
}

const NOT_CALLS: &[&str] = &[
    "if",
    "for",
    "foreach",
    "while",
    "switch",
    "catch",
    "using",
    "lock",
    "return",
    "nameof",
    "typeof",
    "sizeof",
    "default",
    "checked",
    "unchecked",
    "fixed",
    "when",
    "new",
    "base",
    "this",
    "throw",
    "await",
    "in",
    "is",
    "as",
    "out",
    "ref",
    "var",
    "stackalloc",
    "get",
    "set",
];

const NOT_TYPES: &[&str] = &[
    "return",
    "new",
    "await",
    "throw",
    "else",
    "case",
    "using",
    "namespace",
    "class",
    "struct",
    "interface",
    "record",
    "enum",
    "public",
    "private",
    "protected",
    "internal",
    "static",
    "readonly",
    "async",
    "override",
    "virtual",
    "abstract",
    "sealed",
    "partial",
    "const",
    "yield",
    "goto",
    "in",
    "out",
    "ref",
    "is",
    "as",
];

/// Extract the call sites of every method body in `source`.
#[must_use]
pub fn extract(source: &str, declarations: &[CsharpDeclaration]) -> Vec<CsharpCallSite> {
    let masked = mask(source);
    let bytes = masked.as_bytes();
    let types = declared_types(bytes);
    let mut bodies: Vec<(usize, usize, &CsharpDeclaration)> = declarations
        .iter()
        .filter(|d| d.kind == CsharpDeclarationKind::Method)
        .filter_map(|d| body_range(bytes, d.span.end).map(|(s, e)| (s, e, d)))
        .collect();
    // Innermost body wins for a local function: sort so smaller ranges come first.
    bodies.sort_by_key(|(start, end, _)| (end - start, *start));
    let line_starts = crate::line_starts(source);
    let line_of = |offset: usize| match line_starts.binary_search(&offset) {
        Ok(index) => index + 1,
        Err(index) => index,
    };

    let mut sites = Vec::new();
    for (index, byte) in bytes.iter().enumerate() {
        if *byte != b'(' {
            continue;
        }
        let Some(caller) = bodies
            .iter()
            .find(|(start, end, _)| *start <= index && index < *end)
            .map(|(_, _, d)| *d)
        else {
            continue;
        };
        let Some((member, member_start)) =
            identifier_before(bytes, skip_generic_args(bytes, index))
        else {
            continue;
        };
        if NOT_CALLS.contains(&member.as_str()) {
            continue;
        }
        let before = skip_space_back(bytes, member_start);
        if before > 0 && previous_word(bytes, before) == Some("new") {
            continue;
        }
        let receiver = if before > 0 && bytes[before - 1] == b'.' {
            let dot = skip_space_back(bytes, before - 1);
            match identifier_before(bytes, dot) {
                Some((name, start))
                    if start == 0
                        || !matches!(
                            bytes[skip_space_back(bytes, start).saturating_sub(1)],
                            b'.' | b')' | b']'
                        ) =>
                {
                    Some(name)
                }
                _ => Some("?".to_owned()),
            }
        } else if before > 0 && matches!(bytes[before - 1], b'.' | b'?') {
            Some("?".to_owned())
        } else {
            None
        };
        let receiver_type = match receiver.as_deref() {
            Some("this") => caller.semantic_path.last().cloned(),
            Some(name) if name != "?" => types.get(name).cloned().or_else(|| {
                name.chars()
                    .next()
                    .is_some_and(char::is_uppercase)
                    .then(|| name.to_owned())
            }),
            _ => None,
        };
        sites.push(CsharpCallSite {
            caller_key: caller.key.clone(),
            member,
            receiver,
            receiver_type,
            line: line_of(index),
        });
    }
    sites.sort_by(|a, b| {
        (a.line, &a.caller_key, &a.member).cmp(&(b.line, &b.caller_key, &b.member))
    });
    sites
}

/// One base type named in a type declaration's base list (`class A : B, IC`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsharpBaseType {
    /// Key of the declaring type.
    pub type_key: String,
    /// Base name as written, namespace and generic arguments stripped.
    pub base: String,
    /// One-based source line of the base name.
    pub line: usize,
}

/// Base lists of every class, struct, record and interface declaration.
#[must_use]
pub fn base_types(source: &str, declarations: &[CsharpDeclaration]) -> Vec<CsharpBaseType> {
    let masked = mask(source);
    let bytes = masked.as_bytes();
    let line_starts = crate::line_starts(source);
    let line_of = |offset: usize| match line_starts.binary_search(&offset) {
        Ok(index) => index + 1,
        Err(index) => index,
    };
    let mut out = Vec::new();
    for declaration in declarations.iter().filter(|d| {
        matches!(
            d.kind,
            CsharpDeclarationKind::Class
                | CsharpDeclarationKind::Struct
                | CsharpDeclarationKind::Record
                | CsharpDeclarationKind::Interface
        )
    }) {
        let mut index = declaration.span.end;
        // Generic parameters and a record's primary constructor come first.
        loop {
            while index < bytes.len() && bytes[index].is_ascii_whitespace() {
                index += 1;
            }
            match bytes.get(index) {
                Some(b'<') => match matching(bytes, index, b'<', b'>') {
                    Some(close) => index = close + 1,
                    None => break,
                },
                Some(b'(') => match matching(bytes, index, b'(', b')') {
                    Some(close) => index = close + 1,
                    None => break,
                },
                _ => break,
            }
        }
        if bytes.get(index) != Some(&b':') {
            continue;
        }
        index += 1;
        let mut depth = 0i32;
        let mut item_start = index;
        while index <= bytes.len() {
            let byte = bytes.get(index).copied().unwrap_or(b'{');
            let at_where = depth == 0
                && bytes[index..].starts_with(b"where")
                && index > 0
                && bytes[index - 1].is_ascii_whitespace()
                && !bytes.get(index + 5).copied().is_some_and(is_ident);
            match byte {
                b'<' | b'(' => depth += 1,
                b'>' | b')' => depth -= 1,
                _ => {}
            }
            let end_of_list = depth == 0 && (matches!(byte, b'{' | b';') || at_where);
            if (depth == 0 && byte == b',') || end_of_list {
                let item = std::str::from_utf8(&bytes[item_start..index]).unwrap_or_default();
                let head = item.split('<').next().unwrap_or_default().trim();
                let name = head.rsplit('.').next().unwrap_or_default().trim();
                if !name.is_empty() && name.bytes().all(is_ident) {
                    let offset = item_start + item.find(name).unwrap_or(0);
                    out.push(CsharpBaseType {
                        type_key: declaration.key.clone(),
                        base: name.to_owned(),
                        line: line_of(offset),
                    });
                }
                item_start = index + 1;
                if end_of_list {
                    break;
                }
            }
            index += 1;
        }
    }
    out
}

/// A capitalised name used inside a method or type (a candidate type
/// reference). Only the first use per `(source, name)` is kept. Whether the
/// name is a project type is decided later, against the whole project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsharpTypeReference {
    /// Key of the innermost method (signature included) or type using it.
    pub source_key: String,
    /// The name as written, e.g. `WidgetDto`.
    pub name: String,
    /// One-based line of the first use.
    pub line: usize,
}

/// Candidate type references of every method and type body in `source`.
///
/// A name right after `.` is a member access and a name followed by `(` is a
/// call, so both are skipped (`new T(` is kept); declared names and base
/// lists (already `IMPLEMENTS`/`INHERITS`) are not references.
#[must_use]
pub fn type_references(
    source: &str,
    declarations: &[CsharpDeclaration],
) -> Vec<CsharpTypeReference> {
    let masked = mask(source);
    let bytes = masked.as_bytes();
    let line_starts = crate::line_starts(source);
    let line_of = |offset: usize| match line_starts.binary_search(&offset) {
        Ok(index) => index + 1,
        Err(index) => index,
    };
    let mut scopes: Vec<(usize, usize, &CsharpDeclaration)> = Vec::new();
    for declaration in declarations {
        let range = match declaration.kind {
            CsharpDeclarationKind::Method => {
                body_range(bytes, declaration.span.end).map(|(_, end)| {
                    // The signature belongs to the method: start at its line.
                    let line = line_of(declaration.span.start);
                    (line_starts[line - 1], end)
                })
            }
            CsharpDeclarationKind::Namespace => None,
            _ => {
                let open = bytes[declaration.span.end..]
                    .iter()
                    .position(|byte| matches!(byte, b'{' | b';'))
                    .map(|offset| declaration.span.end + offset);
                open.filter(|at| bytes[*at] == b'{')
                    .and_then(|at| matching(bytes, at, b'{', b'}').map(|end| (at, end + 1)))
            }
        };
        if let Some((start, end)) = range {
            scopes.push((start, end, declaration));
        }
    }
    scopes.sort_by_key(|(start, end, _)| (end - start, *start));
    let declared: std::collections::BTreeSet<usize> =
        declarations.iter().map(|d| d.span.start).collect();
    let mut seen: std::collections::BTreeSet<(String, String)> = std::collections::BTreeSet::new();
    let mut out = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if !is_ident(bytes[index]) || (index > 0 && is_ident(bytes[index - 1])) {
            index += 1;
            continue;
        }
        let start = index;
        while index < bytes.len() && is_ident(bytes[index]) {
            index += 1;
        }
        if !bytes[start].is_ascii_uppercase() || declared.contains(&start) {
            continue;
        }
        let before = skip_space_back(bytes, start);
        if before > 0 && bytes[before - 1] == b'.' {
            continue;
        }
        let mut after = index;
        while after < bytes.len() && bytes[after].is_ascii_whitespace() {
            after += 1;
        }
        if bytes.get(after) == Some(&b'(') && previous_word(bytes, before) != Some("new") {
            continue;
        }
        let Some(scope) = scopes
            .iter()
            .find(|(from, to, _)| *from <= start && start < *to)
            .map(|(_, _, d)| *d)
        else {
            continue;
        };
        let name = std::str::from_utf8(&bytes[start..index])
            .unwrap_or_default()
            .to_owned();
        if scope.name == name || scope.semantic_path.last() == Some(&name) {
            continue;
        }
        if seen.insert((scope.key.clone(), name.clone())) {
            out.push(CsharpTypeReference {
                source_key: scope.key.clone(),
                name,
                line: line_of(start),
            });
        }
    }
    out
}

/// Replace comment and string/char literal contents with spaces, keeping
/// byte offsets and newlines.
fn mask(source: &str) -> String {
    let bytes = source.as_bytes();
    let mut out = bytes.to_vec();
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'/' if bytes.get(index + 1) == Some(&b'/') => {
                while index < bytes.len() && bytes[index] != b'\n' {
                    out[index] = b' ';
                    index += 1;
                }
            }
            b'/' if bytes.get(index + 1) == Some(&b'*') => {
                while index < bytes.len()
                    && !(bytes[index] == b'*' && bytes.get(index + 1) == Some(&b'/'))
                {
                    if bytes[index] != b'\n' {
                        out[index] = b' ';
                    }
                    index += 1;
                }
                for offset in 0..2 {
                    if let Some(slot) = out.get_mut(index + offset) {
                        *slot = b' ';
                    }
                }
                index += 2;
            }
            quote @ (b'"' | b'\'') => {
                let verbatim = index > 0 && matches!(bytes[index - 1], b'@')
                    || index > 1 && bytes[index - 1] == b'$' && bytes[index - 2] == b'@';
                index += 1;
                while index < bytes.len() {
                    let byte = bytes[index];
                    if byte == b'\\' && !verbatim {
                        out[index] = b' ';
                        if let Some(slot) = out.get_mut(index + 1) {
                            if *slot != b'\n' {
                                *slot = b' ';
                            }
                        }
                        index += 2;
                        continue;
                    }
                    if byte == quote {
                        if verbatim && bytes.get(index + 1) == Some(&quote) {
                            out[index] = b' ';
                            out[index + 1] = b' ';
                            index += 2;
                            continue;
                        }
                        break;
                    }
                    if byte == b'\n' && !verbatim {
                        break;
                    }
                    if byte != b'\n' {
                        out[index] = b' ';
                    }
                    index += 1;
                }
                index += 1;
            }
            _ => index += 1,
        }
    }
    String::from_utf8(out).unwrap_or_default()
}

/// Body of the method whose name ends at `name_end`: a `{ ... }` block or an
/// `=> ...;` expression. `None` for a declaration without a body.
fn body_range(bytes: &[u8], name_end: usize) -> Option<(usize, usize)> {
    let mut index = name_end;
    // Skip generic parameters and the parameter list.
    while index < bytes.len() && bytes[index] != b'(' {
        if matches!(bytes[index], b'{' | b';' | b'=') {
            return None;
        }
        index += 1;
    }
    index = matching(bytes, index, b'(', b')')? + 1;
    while index < bytes.len() {
        match bytes[index] {
            b'{' => return matching(bytes, index, b'{', b'}').map(|end| (index, end + 1)),
            b';' => return None,
            b'=' if bytes.get(index + 1) == Some(&b'>') => {
                let mut depth = 0i32;
                let mut cursor = index + 2;
                while cursor < bytes.len() {
                    match bytes[cursor] {
                        b'(' | b'{' | b'[' => depth += 1,
                        b')' | b'}' | b']' => depth -= 1,
                        b';' if depth <= 0 => return Some((index, cursor + 1)),
                        _ => {}
                    }
                    cursor += 1;
                }
                return None;
            }
            b'(' => index = matching(bytes, index, b'(', b')')? + 1,
            _ => index += 1,
        }
    }
    None
}

fn matching(bytes: &[u8], open_at: usize, open: u8, close: u8) -> Option<usize> {
    let mut depth = 0i32;
    for (offset, byte) in bytes[open_at..].iter().enumerate() {
        if *byte == open {
            depth += 1;
        } else if *byte == close {
            depth -= 1;
            if depth == 0 {
                return Some(open_at + offset);
            }
        }
    }
    None
}

fn is_ident(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn skip_space_back(bytes: &[u8], mut end: usize) -> usize {
    while end > 0 && bytes[end - 1].is_ascii_whitespace() {
        end -= 1;
    }
    end
}

/// For `Foo<Bar>(`, step back over `<Bar>` so the member is `Foo`.
fn skip_generic_args(bytes: &[u8], paren: usize) -> usize {
    let end = skip_space_back(bytes, paren);
    if end == 0 || bytes[end - 1] != b'>' {
        return end;
    }
    let mut depth = 0i32;
    let mut index = end;
    while index > 0 {
        index -= 1;
        match bytes[index] {
            b'>' => depth += 1,
            b'<' => {
                depth -= 1;
                if depth == 0 {
                    return skip_space_back(bytes, index);
                }
            }
            b'(' | b')' | b';' | b'{' | b'}' | b'=' => return end,
            _ => {}
        }
    }
    end
}

fn identifier_before(bytes: &[u8], end: usize) -> Option<(String, usize)> {
    let end = skip_space_back(bytes, end);
    let mut start = end;
    while start > 0 && is_ident(bytes[start - 1]) {
        start -= 1;
    }
    if start == end || bytes[start].is_ascii_digit() {
        return None;
    }
    let text = std::str::from_utf8(&bytes[start..end]).ok()?.to_owned();
    Some((text, start))
}

fn previous_word(bytes: &[u8], end: usize) -> Option<&str> {
    let mut start = end;
    while start > 0 && is_ident(bytes[start - 1]) {
        start -= 1;
    }
    std::str::from_utf8(&bytes[start..end]).ok()
}

/// `name -> Type` for `Type name` declarations and `var name = new Type(`.
/// A name declared with two different types is dropped as ambiguous.
fn declared_types(bytes: &[u8]) -> BTreeMap<String, String> {
    let mut found: BTreeMap<String, Option<String>> = BTreeMap::new();
    let mut record = |name: String, ty: String| {
        let slot = found.entry(name).or_insert_with(|| Some(ty.clone()));
        if slot.as_deref() != Some(ty.as_str()) {
            *slot = None;
        }
    };
    let mut index = 0;
    while index < bytes.len() {
        if !is_ident(bytes[index])
            || (index > 0 && (is_ident(bytes[index - 1]) || bytes[index - 1] == b'.'))
        {
            index += 1;
            continue;
        }
        let start = index;
        while index < bytes.len() && is_ident(bytes[index]) {
            index += 1;
        }
        let word = std::str::from_utf8(&bytes[start..index]).unwrap_or_default();
        let mut cursor = index;
        if cursor < bytes.len() && bytes[cursor] == b'<' {
            match matching(bytes, cursor, b'<', b'>') {
                Some(close)
                    if !bytes[cursor..close]
                        .iter()
                        .any(|b| matches!(b, b';' | b'(' | b')' | b'{' | b'}' | b'=')) =>
                {
                    cursor = close + 1;
                }
                _ => continue,
            }
        }
        if cursor < bytes.len() && bytes[cursor] == b'?' {
            cursor += 1;
        }
        let gap = cursor;
        while cursor < bytes.len() && matches!(bytes[cursor], b' ' | b'\t') {
            cursor += 1;
        }
        if cursor == gap {
            continue;
        }
        let name_start = cursor;
        while cursor < bytes.len() && is_ident(bytes[cursor]) {
            cursor += 1;
        }
        if cursor == name_start {
            continue;
        }
        let name = std::str::from_utf8(&bytes[name_start..cursor])
            .unwrap_or_default()
            .to_owned();
        let mut after = cursor;
        while after < bytes.len() && matches!(bytes[after], b' ' | b'\t' | b'\r' | b'\n') {
            after += 1;
        }
        let terminator = bytes.get(after).copied();
        if word == "var" {
            if terminator == Some(b'=') {
                let mut probe = after + 1;
                while probe < bytes.len() && bytes[probe].is_ascii_whitespace() {
                    probe += 1;
                }
                if bytes[probe..].starts_with(b"new ") {
                    let ty_start = probe + 4;
                    let mut ty_end = ty_start;
                    while ty_end < bytes.len() && is_ident(bytes[ty_end]) {
                        ty_end += 1;
                    }
                    if ty_end > ty_start {
                        let ty = std::str::from_utf8(&bytes[ty_start..ty_end]).unwrap_or_default();
                        record(name, ty.to_owned());
                    }
                }
            }
            continue;
        }
        let uppercase = word.chars().next().is_some_and(char::is_uppercase);
        if uppercase
            && !NOT_TYPES.contains(&word)
            && matches!(terminator, Some(b';' | b'=' | b',' | b')' | b'{'))
        {
            record(name, word.to_owned());
        }
    }
    found
        .into_iter()
        .filter_map(|(name, ty)| ty.map(|ty| (name, ty)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csharp::declarations::analyze;

    const SOURCE: &str = r#"namespace App;
public class Controller
{
    private readonly IDashboardQuery _query;
    public Controller(IDashboardQuery query) { _query = query; }
    public async Task<int> Get(string id)
    {
        // _ignored.Comment(1);
        var text = "_ignored.InString(2)";
        var helper = new Helper();
        helper.Assist();
        Log("x");
        var value = await _query.Execute(id);
        return Helper.Compute<int>(value).ToString().Length;
    }
    private void Log(string message) => Console.WriteLine(message);
}
"#;

    #[test]
    fn type_references_skip_calls_members_declarations_and_comments() {
        let source = "namespace App;\npublic class Widget : IWidget\n{\n    private Store _store;\n    // Hidden reference\n    public WidgetDto Map(Input input)\n    {\n        var dto = new WidgetDto();\n        Helper.Format(input.Value);\n        Convert(input);\n        return dto;\n    }\n}\n";
        let analysis = analyze("src/Widget.cs", source);
        let refs: Vec<(String, String, usize)> = type_references(source, &analysis.declarations)
            .into_iter()
            .map(|r| {
                let owner = analysis
                    .declarations
                    .iter()
                    .find(|d| d.key == r.source_key)
                    .unwrap();
                (owner.name.clone(), r.name, r.line)
            })
            .collect();
        assert!(
            refs.contains(&("Widget".into(), "Store".into(), 4)),
            "{refs:?}"
        );
        assert!(
            refs.contains(&("Map".into(), "WidgetDto".into(), 6)),
            "{refs:?}"
        );
        assert!(
            refs.contains(&("Map".into(), "Input".into(), 6)),
            "{refs:?}"
        );
        assert!(
            refs.contains(&("Map".into(), "Helper".into(), 9)),
            "{refs:?}"
        );
        assert_eq!(
            refs.iter().filter(|r| r.1 == "WidgetDto").count(),
            1,
            "first use only"
        );
        for absent in ["Hidden", "Value", "Convert", "IWidget", "Format"] {
            assert!(!refs.iter().any(|r| r.1 == absent), "{absent}: {refs:?}");
        }
    }

    #[test]
    fn base_lists_name_each_base_once_without_generics_or_constraints() {
        let source = "namespace App;\npublic class Repo<T> : Base<T>, IRepo, System.IDisposable where T : class\n{\n}\npublic record Point(int X) : IShape;\npublic interface IRepo : IRead\n{\n}\npublic class Plain\n{\n}\n";
        let analysis = analyze("src/Repo.cs", source);
        let bases: Vec<(String, usize)> = base_types(source, &analysis.declarations)
            .into_iter()
            .map(|b| (b.base, b.line))
            .collect();
        assert_eq!(
            bases,
            vec![
                ("Base".to_owned(), 2),
                ("IRepo".to_owned(), 2),
                ("IDisposable".to_owned(), 2),
                ("IShape".to_owned(), 5),
                ("IRead".to_owned(), 6),
            ]
        );
    }

    #[test]
    fn extracts_receivers_types_and_skips_comments_strings_and_constructors() {
        let analysis = analyze("src/Controller.cs", SOURCE);
        let sites = extract(SOURCE, &analysis.declarations);
        let seen: Vec<(String, Option<String>, Option<String>)> = sites
            .iter()
            .map(|s| {
                (
                    s.member.clone(),
                    s.receiver.clone(),
                    s.receiver_type.clone(),
                )
            })
            .collect();
        assert!(
            seen.contains(&(
                "Assist".into(),
                Some("helper".into()),
                Some("Helper".into())
            )),
            "{seen:?}"
        );
        assert!(seen.contains(&("Log".into(), None, None)), "{seen:?}");
        assert!(
            seen.contains(&(
                "Execute".into(),
                Some("_query".into()),
                Some("IDashboardQuery".into())
            )),
            "{seen:?}"
        );
        assert!(
            seen.contains(&(
                "Compute".into(),
                Some("Helper".into()),
                Some("Helper".into())
            )),
            "{seen:?}"
        );
        assert!(
            seen.contains(&(
                "WriteLine".into(),
                Some("Console".into()),
                Some("Console".into())
            )),
            "{seen:?}"
        );
        assert!(
            seen.contains(&("ToString".into(), Some("?".into()), None)),
            "{seen:?}"
        );
        for skipped in ["Comment", "InString", "Helper", "Controller"] {
            assert!(
                !seen.iter().any(|(m, _, _)| m == skipped),
                "{skipped} must not be a call: {seen:?}"
            );
        }
        let get = analysis
            .declarations
            .iter()
            .find(|d| d.name == "Get")
            .unwrap();
        let execute = sites.iter().find(|s| s.member == "Execute").unwrap();
        assert_eq!(execute.caller_key, get.key);
        assert_eq!(execute.line, 13);
    }
}
