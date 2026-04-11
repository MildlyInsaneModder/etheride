use std::sync::{Arc, Mutex};

use vexide::prelude::Motor;

pub struct Drivetrain {
    leftside: Arc<Mutex<[Motor; 3]>>,
    rightside: Arc<Mutex<[Motor; 3]>>,
    gear_ratio: f32,
    wheelsize: f32,
}

impl Drivetrain {
    pub fn new(
        leftside: Arc<Mutex<[Motor; 3]>>,
        rightside: Arc<Mutex<[Motor; 3]>>,
        gear_ratio: f32,
        wheelsize: f32,
    ) -> Self {
        Self {
            leftside,
            rightside,
            gear_ratio,
            wheelsize,
        }
    }
    pub fn get_averaged_position(&self) -> f32 {
        let mut left_positions: Vec<f32> = vec![];
        let mut right_positions: Vec<f32> = vec![];
        const ERR_VAL: f32 = -0.03;
        for motor in self.leftside.lock().unwrap().iter() {
            match motor.position() {
                Ok(val) => left_positions.push(val.as_degrees() as f32),
                Err(_) => left_positions.push(ERR_VAL),
            }
        }
        for motor in self.rightside.lock().unwrap().iter() {
            match motor.position() {
                Ok(val) => right_positions.push(val.as_degrees() as f32),
                Err(_) => right_positions.push(ERR_VAL),
            }
        }
        let mut left_pos: Vec<f32> = vec![];
        for val in left_positions.iter() {
            if *val != ERR_VAL {
                left_pos.push(*val);
            }
        }
        let mut right_pos: Vec<f32> = vec![];
        for val in right_positions.iter() {
            if *val != ERR_VAL {
                right_pos.push(*val);
            }
        }
        let mut right_sum: f32 = 0.0;
        let mut left_sum: f32 = 0.0;
        for val in right_pos.iter() {
            right_sum += val;
        }
        for val in left_pos.iter() {
            left_sum += val;
        }
        (right_sum / right_pos.len() as f32 + right_sum / left_pos.len() as f32) / 2.0
            * self.gear_ratio
    }
    pub fn get_averaged_inches(&self) -> f32 {
        self.get_averaged_position() / self.wheelsize
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
