use crate::CoreError;
use crate::SensoryData;

pub trait Sensor {
    fn capture(&self) -> Result<SensoryData, CoreError>;
}
