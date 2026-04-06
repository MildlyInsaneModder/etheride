use std::time::Duration;

use vexide::{adi::motor, prelude::*};

pub mod manager;
pub mod pid;
pub mod tbh;
use manager::*;
use pid::*;

struct Robot {}

impl Compete for Robot {
    async fn autonomous(&mut self) {
        println!("Autonomous!");
    }

    async fn driver(&mut self) {
        println!("Driver!");
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let mut motor20 = Motor::new(peripherals.port_20, Gearset::Green, Direction::Forward);
    let mut pid = PID::new(PidTune::new(0.5, 0.0, 0.1, 0.0));
    let mut manager = Manager::new(ManagerParams::new(16.0, 20, 40.0, 80, 2000));
    let goal = 500.0;
    while !manager.should_exit() {
        let actual: f32 = motor20.position().unwrap().as_degrees() as f32;
        let output = pid.update(actual, goal);
        manager.update(goal - actual);
        let _ = motor20.set_voltage(output.into());
        vexide::time::sleep(Duration::from_millis(10)).await;
        println!("Position is {}", actual);
    }
    vexide::time::sleep(Duration::from_millis(100)).await;
    let actual: f32 = motor20.position().unwrap().as_degrees() as f32;
    println!("Position is {}", actual);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_adds_two() {
        assert_eq!(2 + 2, 4);
    }
}
