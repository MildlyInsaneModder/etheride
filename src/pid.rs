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
    prev_actual: f32,
    prev_millis: vexide::time::LowResolutionTime,
    prev_error_sign: bool,
    summation: f32,
    first_run: bool,
}

impl PID {
    pub fn new(tune: PidTune) -> Self {
        Self {
            tune,
            prev_actual: 0.0,
            prev_millis: vexide::time::LowResolutionTime::now(),
            prev_error_sign: false,
            summation: 0.0,
            first_run: true,
        }
    }

    pub fn update(&mut self, actual: f32, goal: f32) -> f32 {
        let error = goal - actual;
        if self.first_run {
            self.prev_actual = actual;
            self.prev_millis = vexide::time::LowResolutionTime::now();
            self.prev_error_sign = error.is_sign_positive();
            self.first_run = false;
            return error * self.tune.kp;
        }
        let millis = vexide::time::LowResolutionTime::now();
        let dt = millis.duration_since(self.prev_millis);
        self.prev_millis = millis;
        let dt: f32 = dt.as_millis() as f32 / 1000.0;
        let derivative = (actual - self.prev_actual) / dt;
        if self.prev_error_sign != error.is_sign_positive() {
            self.summation = 0.0;
        }
        if f32::abs(error) <= self.tune.summation_range || self.tune.summation_range == 0.0 {
            self.summation += error * dt;
        } else {
            self.summation = 0.0;
        }
        self.prev_actual = actual;
        self.prev_error_sign = error.is_sign_positive();
        error * self.tune.kp + -derivative * self.tune.kd + self.summation * self.tune.ki
    }
}
