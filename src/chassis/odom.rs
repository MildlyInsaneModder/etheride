use std::sync::{Arc, Mutex, RwLock};

use vexide::{math::Angle, prelude::InertialSensor};

use crate::chassis::tracking_wheel::TrackingWheel;

pub struct Odom {
    inertial_sensor: Arc<Mutex<InertialSensor>>,
    vertical_tracking: Arc<Mutex<TrackingWheel>>,
    horizontal_tracking: Arc<Mutex<TrackingWheel>>,
    vertical_offset: f32,
    horizontal_offset: f32,
    y_position: Arc<RwLock<f32>>,
    x_position: Arc<RwLock<f32>>,
    previous_horizontal_input: f32,
    previous_vertical_input: f32,
    previous_theta: Angle,
}
impl Odom {
    pub fn new(
        vertical_tracking: Arc<Mutex<TrackingWheel>>,
        horizontal_tracking: Arc<Mutex<TrackingWheel>>,
        inertial_sensor: Arc<Mutex<InertialSensor>>,
        vertical_offset: f32,
        horizontal_offset: f32,
    ) -> Self {
        Self {
            inertial_sensor,
            vertical_tracking,
            horizontal_tracking,
            vertical_offset,
            horizontal_offset,
            y_position: Arc::new(RwLock::new(0.0)),
            x_position: Arc::new(RwLock::new(0.0)),
            previous_horizontal_input: 0.0,
            previous_vertical_input: 0.0,
            previous_theta: Angle::from_degrees(0.0),
        }
    }
    pub fn calculate(&mut self) {
        let theta = self
            .inertial_sensor
            .lock()
            .unwrap()
            .heading()
            .unwrap_or(self.previous_theta);
        let horizontal_input = self
            .horizontal_tracking
            .lock()
            .unwrap()
            .get_inches_travelled();
        let vertical_input = self
            .vertical_tracking
            .lock()
            .unwrap()
            .get_inches_travelled();
        let mut vertical_difference: f32 = vertical_input - self.previous_vertical_input;
        let mut horizontal_difference: f32 = horizontal_input - self.previous_horizontal_input;
        let theta_difference = theta - self.previous_theta;
        let cos_theta = theta.cos() as f32;
        let sin_theta = theta.sin() as f32;
        if theta_difference.as_degrees() != 0.0 {
            let sin_half_theta_diff: f32 = theta_difference.sin() as f32;
            horizontal_difference = 2.0
                * sin_half_theta_diff
                * ((horizontal_difference / theta_difference.as_radians() as f32)
                    + self.horizontal_offset);
            vertical_difference = 2.0
                * sin_half_theta_diff
                * ((vertical_difference / theta_difference.as_radians() as f32)
                    + self.vertical_offset);
        }
        let delta_x_position = cos_theta * horizontal_difference + sin_theta * vertical_difference;
        let delta_y_position = -sin_theta * horizontal_difference + cos_theta * vertical_difference;
        *self.y_position.write().unwrap() += delta_y_position;
        *self.x_position.write().unwrap() += delta_x_position;
        self.previous_horizontal_input = horizontal_input;
        self.previous_vertical_input = vertical_input;
        self.previous_theta = theta;
    }
    pub fn get_y_position(&self) -> f32 {
        self.y_position.read().unwrap().clone()
    }
    pub fn get_x_position(&self) -> f32 {
        self.y_position.read().unwrap().clone()
    }
}
