use vexide::{peripherals, prelude::*, smart::SmartPort};
pub struct MotorGroup {
    motors: Vec<Motor>,
}
impl MotorGroup {
    pub fn from_motors(motor_vector: Vec<Motor>) -> Self {
        Self {
            motors: motor_vector
        }
    }
    pub fn new(mut port_nums: Vec<SmartPort>, gearset: Gearset, directions: Vec<Direction>) -> Self {
        let mut motor_vector: Vec<Motor> = vec![];
        for val in 0..=port_nums.len()  {
            let motor = Motor::new(port_nums.remove(val), gearset, directions[val]);
            motor_vector.push(motor);
        }
        Self {
            motors: motor_vector
        }
    }
}