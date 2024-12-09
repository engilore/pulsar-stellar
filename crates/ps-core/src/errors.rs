use std::fmt;

#[derive(Debug)]
pub enum CoreError {
    DeviceNotFound,
    DataCaptureFailed(String),
    UnsupportedModality(String),
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::DeviceNotFound => write!(f, "Device not found"),
            CoreError::DataCaptureFailed(msg) => write!(f, "Data capture failed: {}", msg),
            CoreError::UnsupportedModality(modality) => {
                write!(f, "Unsupported modality: {}", modality)
            }
        }
    }
}

impl std::error::Error for CoreError {}
