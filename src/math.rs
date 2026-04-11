pub fn standard_normal(x: f32) -> f32 {
    (-0.5 * x * x).exp() / (std::f32::consts::TAU).sqrt()
}

pub struct Array {
    pub data: Vec<f32>,
    pub mean: f32,
    pub var: f32,
}

impl Array {
    pub fn new(data: Vec<f32>) -> Array {
        let mean = data.iter().sum::<f32>() / data.len() as f32;
        let var = data.iter().map(|x| (*x - mean).powi(2)).sum::<f32>() / data.len() as f32;
        Array { data, mean, var }
    }
}
