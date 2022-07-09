pub(crate) fn calculate_standard_deviation(variance: f32) {
    println!("The standard deviation is: {}", variance.sqrt());
}

pub(crate) fn calculate_variance(variables: Vec<i32>, media: f32) {
    let mut variance: f32 = 0.0;
    for variable_index in 0..variables.len() {
        variance += f32::powf(variables[variable_index] as f32 - media, 2.0);
    }
    calculate_standard_deviation(variance);
    println!("The variance is {}:", variance);
}

pub(crate) fn calculate_media(variables: Vec<i32>, probabilities: Vec<f32>) {
    let mut media: f32 = 0.0;
    for indexes in 0..variables.len() {
        media += variables[indexes] as f32 * probabilities[indexes];
    }
    calculate_variance(variables, media);
    println!("The media is {}:", media);
}