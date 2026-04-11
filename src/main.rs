use vexide::prelude::*;

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
async fn main(peripherals: Peripherals) {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_adds_two() {
        assert_eq!(2 + 2, 4);
    }
}
