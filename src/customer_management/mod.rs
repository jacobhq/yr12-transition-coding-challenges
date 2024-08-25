use chrono::NaiveDate;
use crate::helpers::read_string_input;

pub struct Customer {
    first_name: String,
    last_name: String,
    date: NaiveDate,
    pub id: String
}

impl Customer {
    pub fn new() -> Self {
        let first_name = read_string_input("Enter your first name:");
        let last_name = read_string_input("Enter your last name:");
        let date = read_string_input("Enter today's date (DD/MM/YYYY):");
        let date_str: &str = &date;

        let date = match NaiveDate::parse_from_str(date_str.trim(), "%d/%m/%Y") {
            Ok(date) => date,
            Err(e) => panic!("{}", e)
        };

        let id = Self::generate_customer_id(&first_name, &last_name, date);

        Customer {
            first_name,
            last_name,
            date,
            id
        }
    }

    fn generate_customer_id(first_name: &str, last_name: &str, date: NaiveDate) -> String {
        let formatted_date = NaiveDate::format(&date, "%Y%m%d").to_string();
        let three_letters_last_name = last_name.chars().take(3).collect::<String>().to_uppercase();
        let first_f_name = first_name.chars().take(1).collect::<String>().to_uppercase();
        let len_f_name = first_name.len();
        
        format!("{}{}{}{}", formatted_date, three_letters_last_name, first_f_name, len_f_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() {
        let first_name = "John";
        let last_name = "Doe";
        let date = NaiveDate::parse_from_str("25/08/2024", "%d/%m/%Y").unwrap();
        
        assert_eq!(Customer::generate_customer_id(first_name, last_name, date), "20240825DOEJ4")
    }

    #[test]
    fn short_names() {
        let first_name = "J";
        let last_name = "D";
        let date = NaiveDate::parse_from_str("25/08/2024", "%d/%m/%Y").unwrap();

        assert_eq!(Customer::generate_customer_id(first_name, last_name, date), "20240825DJ1")
    }
}