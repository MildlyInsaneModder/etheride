use crate::controllers;
use std::sync::{Arc, Mutex};

use vexide::prelude::Motor;

pub struct Drivetrain {
    leftside: Arc<Mutex<[Motor; 3]>>,
    rightside: Arc<Mutex<[Motor; 3]>>,
    gear_ratio: f32,
    wheel_radius: f32,
}

impl Drivetrain {
    pub fn new(
        leftside: Arc<Mutex<[Motor; 3]>>,
        rightside: Arc<Mutex<[Motor; 3]>>,
        gear_ratio: f32,
        wheel_radius: f32,
    ) -> Self {
        Self {
            leftside,
            rightside,
            gear_ratio,
            wheel_radius,
        }
    }
    pub fn get_averaged_position(&self) -> f32 {
        //Need to build in feedback system to avoid reconnecting motors screwing with values
        let mut left_positions: Vec<f32> = vec![];
        let mut right_positions: Vec<f32> = vec![];
        for motor in self.leftside.lock().unwrap().iter() {
            if let Ok(val) = motor.position() {
                left_positions.push(val.as_degrees() as f32)
            }
        }
        for motor in self.rightside.lock().unwrap().iter() {
            if let Ok(val) = motor.position() {
                right_positions.push(val.as_degrees() as f32)
            }
        }
        let mut right_sum: f32 = 0.0;
        let mut left_sum: f32 = 0.0;
        for val in left_positions.iter() {
            right_sum += val;
        }
        for val in right_positions.iter() {
            left_sum += val;
        }
        (right_sum / right_positions.len() as f32 + left_sum / left_positions.len() as f32) / 2.0
            * self.gear_ratio
    }
    pub fn get_averaged_inches(&self) -> f32 {
        self.get_averaged_position() / 360.0 * 2.0 * self.wheel_radius * std::f32::consts::PI
    }
    pub fn set_voltage(&mut self, volts_left: f32, volts_right: f32) {
        for motor in self.rightside.lock().unwrap().iter_mut() {
            let _ = motor.set_voltage(volts_right.into());
        }
        for motor in self.leftside.lock().unwrap().iter_mut() {
            let _ = motor.set_voltage(volts_left.into());
        }
    }
}
