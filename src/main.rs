mod customer_management;

fn main() {
    let customer = customer_management::Customer::new();
    println!("Customer ID: {}", customer.id)
}
