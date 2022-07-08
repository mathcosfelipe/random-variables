pub(crate) fn calculate_variance(variables: Vec<f32>, media: f32) {
    let mut variance: f32 = 0.0;
    for variable_index in 0..variables.len() {
        // variance += variables[variable_index] - media;
        variance += variables[variable_index] - media;
    }
}

pub(crate) fn calculate_media(variables: Vec<f32>, probabilities: Vec<f32>) {
    let mut media: f32 = 0.0;
    for indexes in 0..variables.len() {
        media += variables[indexes] * probabilities[indexes];
    }
    calculate_variance(variables, media);
    println!("{}", media);
}