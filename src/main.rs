extern crate beginner_tools;
use beginner_tools::input;

mod resum;

fn main() {

    let variable_name: String = loop {
        if let Ok(variable_name_validity) = input("Input the name of variable: ") {
            break variable_name_validity
        };
        println!("Error! Invalid value. Try again.");
        main();
    };

    let amount: u32 = loop {
        let variable_name_input: String = format!("Input the amount of variables {}: ", variable_name);
        if let Ok(amount_validity) = input(&variable_name_input) {
            break amount_validity
        };
        println!("Invalid value! Input a integer number. Try again.");
    };

    let mut variables: Vec<i32> = Vec::new();
    let mut probabilities: Vec<f32> = Vec::new();
    
    if amount > 0 {
        for variable in 1..amount + 1 {
            let variable_value: i32 = loop {
                let input_variable_message: String = format!("Input the value of variable {}{}: ", variable_name, variable);
                if let Ok(variable_value_validity) = input(&input_variable_message) {
                    break variable_value_validity
                };
                println!("Invalid value! Input a float number. Try again.");
            };
            variables.push(variable_value);
        }

        for probability in 0..variables.len() {
            let probability_value: f32 = loop {
                let input_probability_message: String = format!("Input the probability of variable {}{}: ", variable_name, variables[probability]);
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
            resum::calculate_media(variables, probabilities);
        }
    
    } else {
        println!("You must enter at least one variable. Try again.");
        main();
    }

}