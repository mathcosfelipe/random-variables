extern crate beginner_tools;
use beginner_tools::input;

mod calculate_resum;

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
        for variable in 1..amount + 1 {
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

        let mut total_probabilities: f32 = 0.0;

        for probability_index in 0..probabilities.len() {
            total_probabilities += probabilities[probability_index];
        }

        if total_probabilities != 1.0 {
            println!("Error! The sum of probabilities mut be 1.0. Try again.");
            main();
        } else {
            calculate_resum::calculate_media(variables, probabilities);
        }
    
    } else {
        println!("You must enter at least one variable. Try again.");
        main();
    }

}