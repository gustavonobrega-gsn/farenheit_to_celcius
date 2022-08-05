use std::io;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

fn farenheit_to_celcius(temperature: Decimal) -> Decimal {
    ((temperature- dec!(32)) * (dec!(5) / dec!(9))).round_dp(2)
}

fn main() {

    loop {

        println!("Enter Fahrenheit:");

        let mut temperature = String::new();

        io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");
        
        let mut temperature: Decimal = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        temperature = farenheit_to_celcius(temperature);

        println!("In Celsius: {temperature}");
    }   
}
