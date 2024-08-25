mod customer_management;
mod helpers;

fn main() {
    let customer = customer_management::Customer::new();
    println!("Customer ID: {}", customer.id)
}
