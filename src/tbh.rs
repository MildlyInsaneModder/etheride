/// Take-Back-Half (TBH) controller.
///
/// TBH is a simple, tuning-friendly algorithm popular in VEX robotics. The
/// controller accumulates an output value proportional to the error. Each time
/// the error crosses zero (sign change), the gain `k` is halved, which
/// dampens overshoot without requiring careful manual tuning.
///
/// When a new goal is set, `k` is reset to its initial value so the
/// controller responds aggressively to the new target.
pub struct TBH {
    prev_goal: Option<f32>,
    k: f32,
    kinit: f32,
    prev_output: Option<f32>,
    prev_sign: Option<bool>,
}

impl TBH {
    /// Create a new TBH controller.
    ///
    /// `kinit` is the initial (and reset) gain applied to the error each
    /// iteration. A good starting point is a value that, when multiplied by
    /// the maximum expected error, produces the maximum desired output.
    pub fn new(kinit: f32) -> Self {
        Self {
            prev_goal: None,
            k: kinit,
            kinit,
            prev_output: None,
            prev_sign: None,
        }
    }

    /// Compute the controller output for the current measurement and setpoint.
    ///
    /// Returns a voltage value (f32) suitable for
    /// [`Motor::set_voltage`](vexide::devices::smart::motor::Motor::set_voltage).
    pub fn update(&mut self, actual: f32, goal: f32) -> f32 {
        let error = goal - actual;

        if Some(error.is_sign_positive()) != self.prev_sign {
            self.k *= 0.5;
        }
        self.prev_sign = Some(error.is_sign_positive());

        if self.prev_goal != Some(goal) {
            self.k = self.kinit;
        }
        self.prev_goal = Some(goal);

        let prev_output = self.prev_output.unwrap_or(0.0);
        let output = prev_output + self.k * error;
        self.prev_output = Some(output);
        output
    }
}
