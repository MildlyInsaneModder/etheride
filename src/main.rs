use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use vexide::prelude::*;

use crate::chassis::drivetrain::{self, Drivetrain};

pub mod log;
pub mod manager;

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
    let drivetrain =
        drivetrain::Drivetrain::new(leftside.clone(), rightside.clone(), 0.75 / 1.0, 3.25 / 2.0);
    //drivetrain.set_voltage(2.0, 2.0);
    loop {
        println!("Pos is{}", drivetrain.get_averaged_inches());
        vexide::time::sleep(Duration::from_millis(10)).await;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_adds_two() {
        assert_eq!(2 + 2, 4);
    }
}
