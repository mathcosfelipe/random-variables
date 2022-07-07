extern crate beginner_tools;
use beginner_tools::input;

fn main() {

    let amount: u32 = loop {
        if let Ok(amount_validity) = input("Input the amount of variables (x): ") {
            break amount_validity
        };
        println!("Invalid value! Input a integer number. Try again.");
    };

    let mut variables: Vec<f32> = Vec::new();
    let mut probabilities: Vec<f32> = Vec::new();
    
    if amount > 0 {
    
        for variable in 1..amount {
            let variable_value: f32 = loop {
                let input_variable_message: String = format!("Input the value of variable x{}: ", variable);
                if let Ok(variable_value_validity) = input(&input_variable_message) {
                    break variable_value_validity
                };
                println!("Invalid value! Input a float number. Try again.");
            };
            variables.push(variable_value);
        }

        for probability in 0..variables.len() {
            let probability_value: f32 = loop {
                let input_probability_message: String = format!("Input the probability of variable x{}: ", variables[probability]);
                if let Ok(probability_value_validity) = input(&input_probability_message) {
                    break probability_value_validity
                };
                println!("Invalid value! Input a float number. Try again.");
            };
            probabilities.push(probability_value);
        }
    
    } else {
        println!("You must enter at least one variable. Try again.");
        main();
    }

    println!("{:?}", variables);
    println!("{:?}", probabilities);

}