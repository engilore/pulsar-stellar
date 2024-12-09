#[derive(Debug)]
pub struct SensoryData {
    pub modality: Modality,
    pub timestamp: u64,
    pub data: Vec<f64>,
}

#[derive(Debug)]
pub enum Modality {
    Vision,
    Acoustic,
    Thermal,
    Custom(String),
}
