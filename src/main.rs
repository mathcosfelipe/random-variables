extern crate beginner_tools;
use beginner_tools::input;

fn main() {

    let amount: u32 = loop {
        if let Ok(amount_validity) = input("Input the amount of variables (x): ") {
            break amount_validity
        };
        println!("Invalid value! Input a integer number. Try again.");
    };
    
    if amount > 0 {
    
        for variable in 1..amount {
            let variable_value: u32 = loop {
                if let Ok(variable_value_validity) = input("Input the value of variable : ") {
                    break variable_value_validity
                };
                println!("Invalid value! Input a integer number. Try again.");
            };
        }
    
    } else {
        println!("You must enter at least one variable. Try again.");
        main();
    }

}