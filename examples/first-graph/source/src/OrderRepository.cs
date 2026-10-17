namespace Demo.Shop;

public class OrderRepository {
    public Order Find(string number) {
        return new Order { Number = number, Total = 42 };
    }
}
