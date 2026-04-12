use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use vexide::prelude::*;

use crate::{
    chassis::{
        Chassis,
        drivetrain::{self, Drivetrain},
        odom,
    },
    controllers::PidTune,
    manager::ManagerParams,
    utils::wrap_180,
};

pub mod log;
pub mod manager;
pub mod utils;

struct Robot {}

impl Compete for Robot {
    async fn autonomous(&mut self) {
        println!("Autonomous!");
    }

    async fn driver(&mut self) {
        println!("Driver!");
    }
}
pub mod chassis;
pub mod controllers;

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let hori_sensor = Arc::new(Mutex::new(RotationSensor::new(
        peripherals.port_19,
        Direction::Forward,
    )));
    let vert_sensor = Arc::new(Mutex::new(RotationSensor::new(
        peripherals.port_6,
        Direction::Reverse,
    )));
    let hori_tracking = Arc::new(Mutex::new(chassis::tracking_wheel::TrackingWheel::new(
        hori_sensor,
        2.0,
    )));
    let vert_tracking = Arc::new(Mutex::new(chassis::tracking_wheel::TrackingWheel::new(
        vert_sensor,
        2.0,
    )));
    let leftside = Arc::new(Mutex::new([
        Motor::new(peripherals.port_1, Gearset::Blue, Direction::Reverse),
        Motor::new(peripherals.port_2, Gearset::Blue, Direction::Reverse),
        Motor::new(peripherals.port_3, Gearset::Blue, Direction::Reverse),
    ]));
    let rightside = Arc::new(Mutex::new([
        Motor::new(peripherals.port_8, Gearset::Blue, Direction::Forward),
        Motor::new(peripherals.port_9, Gearset::Blue, Direction::Forward),
        Motor::new(peripherals.port_10, Gearset::Blue, Direction::Forward),
    ]));
    let mut inertial_sensor = InertialSensor::new(peripherals.port_20);
    let _ = inertial_sensor.calibrate().await;
    let inertial_sensor = Arc::new(Mutex::new(inertial_sensor));
    {}
    let odom = Arc::new(Mutex::new(odom::Odom::new(
        vert_tracking.clone(),
        hori_tracking.clone(),
        inertial_sensor.clone(),
        0.0,
        0.0,
    )));
    let drivetrain = Arc::new(Mutex::new(Drivetrain::new(
        leftside.clone(),
        rightside.clone(),
        0.75 / 1.0,
        3.25 / 2.0,
    )));
    //drivetrain.set_voltage(2.0, 2.0);
    let mut chassis = Chassis::new(odom.clone(), drivetrain.clone(), inertial_sensor.clone());
    chassis.set_angular_tune(PidTune::new(0.2, 0.0, 0.0, 0.0));
    chassis.set_angular_params(ManagerParams::new(0.5, 40, 1.0, 100, 4000));

    //chassis.drive_for(10.0).await;
    println!("Here");
    chassis.turn_to(10.0).await;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_adds_two() {
        assert_eq!(2 + 2, 4);
    }
}
