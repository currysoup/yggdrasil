// Communication with the sensor

pub mod serial;

pub trait Comms {
    fn req_moisture(&mut self, plant_id: PlantId) -> MoistureLevel;
}

enum RequestType {
    Moisture,
    Temperature,
}

#[derive(Debug)]
pub struct PlantId(pub u32);

#[derive(Debug)]
pub struct MoistureLevel {
    plant_id: PlantId,
    /// Moisture levels are measured from 0.0 (very dry) to 1.0 (very wet)
    level: i16,
}

impl MoistureLevel {
    pub fn new(plant_id: PlantId, level: i16) -> MoistureLevel {
        MoistureLevel {
            plant_id: plant_id,
            level: level,
        }
    }
}
