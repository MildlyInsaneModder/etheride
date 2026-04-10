use vexide::{
    peripherals,
    prelude::*,
    smart::{PortError, SmartPort},
};
pub struct MotorGroup {
    motors: Vec<Motor>,
}
impl MotorGroup {
    pub fn from_motors(motor_vector: Vec<Motor>) -> Self {
        Self {
            motors: motor_vector,
        }
    }
    pub fn new(
        mut port_nums: Vec<SmartPort>,
        gearset: Gearset,
        directions: Vec<Direction>,
    ) -> Self {
        let mut motor_vector: Vec<Motor> = vec![];
        (0..port_nums.len()).for_each(|val| {
            let motor = Motor::new(port_nums.remove(val), gearset, directions[val]);
            motor_vector.push(motor);
        });
        Self {
            motors: motor_vector,
        }
    }
    pub fn set_voltage(&mut self, volts: f64) -> Result<(), PortError> {
        let mut err: Result<(), PortError> = Ok(());
        for val in 0..self.motors.len() {
            if err != Ok(()) {
                let _ = self.motors[val].set_voltage(volts);
            }
            err = self.motors[val].set_voltage(volts);
        }

        err
    }
}
