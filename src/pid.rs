pub struct PidTune {
    kp: f32,
    ki: f32,
    kd: f32,
    summation_range: f32,
}

impl PidTune {
    pub fn new(kp: f32, ki: f32, kd: f32, summation_range: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            summation_range,
        }
    }
} //Impl end

pub struct PID {
    pub tune: PidTune,
    prev_actual: Option<f32>,
    prev_millis: Option<vexide::time::LowResolutionTime>,
    prev_error_sign: Option<bool>,
    summation: f32,
}

impl PID {
    pub fn new(tune: PidTune) -> Self {
        Self {
            tune,
            prev_actual: None,
            prev_millis: None,
            prev_error_sign: None,
            summation: 0.0,
        }
    }

    pub fn update(&mut self, actual: f32, goal: f32) -> f32 {
        let error = goal - actual;
        let millis = vexide::time::LowResolutionTime::now();
        let mut dt: f32;
        if self.prev_millis.is_none() {
            dt = 1000.0;
        } else {
            dt = millis.duration_since(self.prev_millis.unwrap()).as_millis() as f32;
        }
        self.prev_millis = Some(millis);
        dt /= 1000.0;
        let derivative = actual - self.prev_actual.unwrap_or(actual);
        if self.prev_error_sign != Some(error.is_sign_positive()) {
            self.summation = 0.0;
        }
        if f32::abs(error) <= self.tune.summation_range || self.tune.summation_range == 0.0 {
            self.summation += error * dt;
        } else {
            self.summation = 0.0;
        }
        self.prev_actual = Some(actual);
        self.prev_error_sign = Some(error.is_sign_positive());
        error * self.tune.kp + -derivative * self.tune.kd + self.summation * self.tune.ki
    }
}
