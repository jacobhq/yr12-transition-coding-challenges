use crate::helpers::{read_string_input, read_f32_input};

struct Product {
    name: String,
    cost_in_pounds: f32
}

impl Clone for Product {
    fn clone(&self) -> Self {
        Product {
            name: self.name.clone(),
            cost_in_pounds: self.cost_in_pounds
        }
    }
}

struct Order {
    products: Vec<Product>,
    total_cost: f32
}

struct OrderInsights {
    max: Product,
    min: Product
}

impl Order {
    pub fn new() -> Self {
        let mut product_name: String = String::from("");
        let mut cost_in_pounds: f32;
        
        let mut products = Vec::new();

        while product_name != "None" {
            product_name = read_string_input("Name of product (type \"None\" to finish):");
            cost_in_pounds = read_f32_input("Cost of product (GBP):");
            
            if (cost_in_pounds > 50f32) { cost_in_pounds *= 0.95f32 };
            
            products.push(Product {
                name: product_name.clone(),
                cost_in_pounds
            });
        }
        
        let total_cost = Self::calculate_total_cost(products.clone());
        
        Order {
            products,
            total_cost
        }
    }
    
    fn calculate_total_cost(products: Vec<Product>) -> f32 {
        let mut total_cost: f32 = 0f32;
        
        for i in products.iter() {
            total_cost += i.cost_in_pounds
        }
        
        total_cost *= 1.2;
        
        total_cost
    }
    
    //
    // pub fn calculate_order_insights(&self) -> OrderInsights {
    //     
    // }
}