mod customer_management;
mod helpers;
mod order;

fn main() {
    println!("Task 1: Customer ID Generation");
    let customer = customer_management::Customer::new();
    println!("Customer ID: {}", customer.id);
    println!("Task 1 completed");
    println!();
    println!("Task 2: Product Analytics");
    let order = order::Order::new();
    let analytics = order.calculate_order_insights();
    println!("The most expensive product was {}, which cost £{:.2}", analytics.max.name, analytics.max.cost_in_pounds);
    println!("The cheapest product was {}, which cost £{:.2}", analytics.min.name, analytics.min.cost_in_pounds);
    println!("Including VAT, your order cost £{:.2}", order.total_cost);
    println!("Task 2 completed");
}
