use std::sync::{Arc, Mutex, RwLock};

use vexide::{math::Angle, prelude::InertialSensor};

use crate::chassis::{
    drivetrain::{self, Drivetrain},
    tracking_wheel::TrackingWheel,
};

pub struct Odom {
    imu: Arc<Mutex<InertialSensor>>,
    vertical_tracking: Arc<Mutex<TrackingWheel>>,
    horizontal_tracking: Arc<Mutex<TrackingWheel>>,
    drivetrain: Arc<Mutex<Drivetrain>>,
    vertical_offset: f32,
    horizontal_offset: f32,
    pub y_position: Arc<RwLock<f32>>,
    pub x_position: Arc<RwLock<f32>>,
    previous_horizontal_input: f32,
    previous_vertical_input: f32,
    previous_theta: Angle,
    previous_drivetrain_state: (f32, f32),
}
impl Odom {
    pub fn new(
        vertical_tracking: Arc<Mutex<TrackingWheel>>,
        horizontal_tracking: Arc<Mutex<TrackingWheel>>,
        inertial_sensor: Arc<Mutex<InertialSensor>>,
        drivetrain: Arc<Mutex<Drivetrain>>,
        vertical_offset: f32,
        horizontal_offset: f32,
        drivetrain_state: (f32, f32),
    ) -> Self {
        Self {
            previous_drivetrain_state: (
                drivetrain_state.0,
                drivetrain_state.1,
            ),
            imu: inertial_sensor,
            vertical_tracking,
            drivetrain,
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
    pub fn calculate_drivetrain_theta(&self) -> Angle {
        let right_delta =
            self.drivetrain.lock().unwrap().get_right_inches() - self.previous_drivetrain_state.1;
        let left_delta =
            self.drivetrain.lock().unwrap().get_left_inches() - self.previous_drivetrain_state.0;
        //Left is larger is 0, Right is larger is 1
        //let side = right_delta > left_delta;
        let delta_theta = if true {
            let longer = left_delta - right_delta;
            Angle::from_radians(
                (-(longer / self.drivetrain.lock().unwrap().wheelbase).atan()).into(),
            )
        } else {
            let longer = right_delta - left_delta;
            Angle::from_radians((longer / self.drivetrain.lock().unwrap().wheelbase).atan().into())
        };
        self.previous_theta + delta_theta
        
    }
    pub fn calculate(&mut self) {
        let theta = self
            .imu
            .lock()
            .unwrap()
            .heading()
            .unwrap_or(self.calculate_drivetrain_theta());
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
        self.previous_drivetrain_state.0 = self.drivetrain.lock().unwrap().get_left_inches();
        self.previous_drivetrain_state.1 = self.drivetrain.lock().unwrap().get_right_inches();
    }
    pub fn get_y_position(&self) -> f32 {
        *self.y_position.read().unwrap()
    }
    pub fn get_x_position(&self) -> f32 {
        *self.x_position.read().unwrap()
    }
    pub fn get_theta_deg(&self) -> f32 {
        self.imu
            .lock()
            .unwrap()
            .heading()
            .unwrap_or(self.calculate_drivetrain_theta())
            .as_degrees() as f32
    }
    pub fn get_theta_rad(&self) -> f32 {
        self.imu
            .lock()
            .unwrap()
            .heading()
            .unwrap_or(self.calculate_drivetrain_theta())
            .as_radians() as f32
    }
}
