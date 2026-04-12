use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use vexide::math::Angle;
use vexide::prelude::InertialSensor;

use crate::chassis::{drivetrain::Drivetrain, odom::Odom};
use crate::controllers::*;
use crate::manager;
use crate::manager::*;

pub mod drivetrain;
pub mod odom;
pub mod tracking_wheel;
struct Chassis {
    odom: Arc<Mutex<Odom>>,
    drivetrain: Arc<Mutex<Drivetrain>>,
    imu: Arc<Mutex<InertialSensor>>,
    linear_tune: PidTune,
    angular_tune: PidTune,
    correction_tune: PidTune,
    exit_params: ManagerParams,
}

impl Chassis {
    pub fn new(
        odom: Arc<Mutex<Odom>>,
        drivetrain: Arc<Mutex<Drivetrain>>,
        imu: Arc<Mutex<InertialSensor>>,
    ) -> Self {
        Self {
            odom,
            drivetrain,
            imu,
            linear_tune: PidTune::new(0.0, 0.0, 0.0, 0.0),
            angular_tune: PidTune::new(0.0, 0.0, 0.0, 0.0),
            correction_tune: PidTune::new(0.0, 0.0, 0.0, 0.0),
            exit_params: ManagerParams::new(0.0, 0, 0.0, 0, 0),
        }
    }
    pub async fn drive_for(&mut self, inches: f32) {
        let original_theta = loop {
            if let Ok(theta) = self.imu.lock().unwrap().heading() {
                break theta;
            }
        };
        let original_position = { self.drivetrain.lock().unwrap().get_averaged_inches() };
        let goal = original_position + inches;
        let mut manager = Manager::new(self.exit_params.clone());
        let mut linear_controller = PID::new(self.linear_tune.clone());
        let mut correction_controller = PID::new(self.correction_tune.clone());
        manager.update(goal - original_position);
        while !manager.should_exit() {
            let position = self.drivetrain.lock().unwrap().get_averaged_inches();
            let linear_output = linear_controller.update(position, goal);
            let theta_delta: f32;
            if let Ok(angle) = self.imu.lock().unwrap().heading() {
                theta_delta = (original_theta.as_degrees() - angle.as_degrees()) as f32;
            } else {
                theta_delta = 0.0;
            }
            let correction_output = correction_controller.update(theta_delta, goal);
            self.drivetrain.lock().unwrap().set_voltage(
                linear_output + correction_output,
                linear_output - correction_output,
            );
            manager.update(goal - position);
            vexide::time::sleep(Duration::from_millis(10)).await;
        }
    }
    pub fn set_linear_tune(&mut self, tune: PidTune) {
        self.linear_tune = tune;
    }
    pub fn set_angular_tune(&mut self, tune: PidTune) {
        self.angular_tune = tune;
    }
    pub fn set_correction_tune(&mut self, tune: PidTune) {
        self.correction_tune = tune;
    }
}
