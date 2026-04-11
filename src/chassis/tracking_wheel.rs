use std::f32::consts::PI;
use std::sync::Arc;
use std::sync::Mutex;
use vexide::prelude::RotationSensor;

pub struct TrackingWheel {
    rotation_sensor: Arc<Mutex<RotationSensor>>,
    offset: f32,
    diameter: f32,
    prev_pos: f32,
}
impl TrackingWheel {
    pub fn new(rotation_sensor: Arc<Mutex<RotationSensor>>, offset: f32, diameter: f32) -> Self {
        rotation_sensor.lock().unwrap().reset_position();
        Self {
            rotation_sensor,
            offset,
            diameter,
            prev_pos: 0.0,
        }
    }
    pub fn get_position(&mut self) -> f32 {
        if let Ok(val) = self.rotation_sensor.lock().unwrap().position() {
            self.prev_pos = val.as_degrees() as f32
        }
        self.prev_pos
    }
    pub fn get_inches_travelled(&mut self) -> f32 {
        self.get_position() / 360.0 * self.diameter * PI
    }
}
