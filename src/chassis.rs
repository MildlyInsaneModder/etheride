use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use vexide::display::Font;
use vexide::math::Point2;
use vexide::prelude::InertialSensor;

use crate::chassis::{drivetrain::Drivetrain, odom::Odom};
use crate::controllers::*;
use crate::manager::*;
use crate::utils::wrap_180;

pub mod drivetrain;
pub mod odom;
pub mod tracking_wheel;
pub struct Chassis {
    odom: Arc<Mutex<Odom>>,
    drivetrain: Arc<Mutex<Drivetrain>>,
    imu: Arc<Mutex<InertialSensor>>,
    linear_tune: PidTune,
    angular_tune: PidTune,
    correction_tune: PidTune,
    linear_params: ManagerParams,
    angular_params: ManagerParams,
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
            linear_params: ManagerParams::new(0.0, 0, 0.0, 0, 0),
            angular_params: ManagerParams::new(0.0, 0, 0.0, 0, 0),
        }
    }
    pub async fn drive_for(&self, inches: f32) {
        let original_theta = loop {
            if let Ok(theta) = self.imu.lock().unwrap().heading() {
                break theta;
            }
        };
        println!("Hello1");
        vexide::time::sleep(Duration::from_millis(2000)).await;
        let original_position = { self.drivetrain.lock().unwrap().get_averaged_inches() };
        let goal = original_position + inches;
        let mut manager = Manager::new(self.linear_params.clone());
        let mut linear_controller = PID::new(self.linear_tune.clone());
        let mut correction_controller = PID::new(self.correction_tune.clone());
        manager.update(goal - original_position);
        while !manager.should_exit() {
            let position = self.drivetrain.lock().unwrap().get_averaged_inches();
            let linear_output = linear_controller.update(goal - position);
            let theta_delta: f32;
            if let Ok(angle) = self.imu.lock().unwrap().heading() {
                theta_delta =
                    wrap_180(original_theta.as_degrees() as f32 - angle.as_degrees() as f32);
            } else {
                theta_delta = 0.0;
            }
            let correction_output = correction_controller.update(theta_delta);
            println!("{}", correction_output);
            self.drivetrain.lock().unwrap().set_voltage(
                linear_output + correction_output,
                linear_output - correction_output,
            );
            manager.update(goal - position);
            vexide::time::sleep(Duration::from_millis(10)).await;
        }
        self.drivetrain.lock().unwrap().set_voltage(0.0, 0.0);
    }
    pub async fn turn_to(&self, degrees: f32) {
        let mut manager = Manager::new(self.angular_params.clone());
        let mut angular_controller = PID::new(self.angular_tune.clone());
        let theta_delta =
            wrap_180(degrees - self.imu.lock().unwrap().heading().unwrap().as_degrees() as f32);
        manager.update(theta_delta);
        while !manager.should_exit() {
            let heading = self.imu.lock().unwrap().heading().unwrap().as_degrees() as f32;
            let theta_delta = wrap_180(degrees - heading);
            manager.update(theta_delta);
            let angular_output = angular_controller.update(theta_delta);
            self.drivetrain
                .lock()
                .unwrap()
                .set_voltage(angular_output, -angular_output);
            vexide::time::sleep(Duration::from_millis(10)).await;
        }
        self.drivetrain.lock().unwrap().set_voltage(0.0, 0.0);
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
    pub fn set_linear_params(&mut self, params: ManagerParams) {
        self.linear_params = params;
    }
    pub fn set_angular_params(&mut self, params: ManagerParams) {
        self.angular_params = params;
    }
}
