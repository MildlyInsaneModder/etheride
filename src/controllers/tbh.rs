pub struct TBH {
    prev_goal: Option<f32>,
    k: f32,
    kinit: f32,
    prev_output: Option<f32>,
    prev_sign: Option<bool>,
}

impl TBH {
    pub fn new(kinit: f32) -> Self {
        Self {
            prev_goal: None,
            k: kinit,
            kinit,
            prev_output: None,
            prev_sign: None,
        }
    }

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
