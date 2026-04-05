use std::time::Duration;

use pid::*;
use vexide::prelude::*;
pub mod pid;

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
    let mut val = PID::new(PidTune::new(3.0, 2.0, 0.0, 5.0));
    let mut position = 0.0;
    let mut output;
    loop {
        output = val.update(position, 90.0);
        position += output * 0.05;
        println!("position is {}", position);
        vexide::time::sleep(Duration::from_millis(20)).await;
        if position > 88.0 && position < 92.0 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_adds_two() {
        assert_eq!(2 + 2, 4);
    }
}

