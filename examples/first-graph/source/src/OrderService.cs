namespace Demo.Shop;

public class OrderService {
    public Order GetOrder(string number) {
        return new Order { Number = number, Total = 42 };
    }

    public bool IsValid(Order order) {
        return order.Total > 0;
    }
}
