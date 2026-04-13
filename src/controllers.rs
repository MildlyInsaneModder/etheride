#[derive(Clone)]
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
    prev_error: Option<f32>,
    prev_millis: Option<vexide::time::LowResolutionTime>,
    prev_error_sign: Option<bool>,
    summation: f32,
}

impl PID {
    pub fn new(tune: PidTune) -> Self {
        Self {
            tune,
            prev_error: None,
            prev_millis: None,
            prev_error_sign: None,
            summation: 0.0,
        }
    }
    pub fn update(&mut self, error: f32) -> f32 {
        let millis = vexide::time::LowResolutionTime::now();
        let mut dt: f32;
        match self.prev_millis {
            None => dt = 1000.0,
            Some(time) => {
                dt = millis.duration_since(time).as_millis() as f32;
            }
        }
        self.prev_millis = Some(millis);
        dt /= 1000.0;
        let prev_error = self.prev_error.unwrap_or(error);
        let derivative = if dt > 0.0 {
            (error - prev_error) / dt
        } else {
            0.0
        };
        if self.prev_error_sign != Some(error.is_sign_positive()) {
            self.summation = 0.0;
        }
        if f32::abs(error) <= self.tune.summation_range || self.tune.summation_range == 0.0 {
            self.summation += error * dt;
        } else {
            self.summation = 0.0;
        }
        self.prev_error = Some(error);
        self.prev_error_sign = Some(error.is_sign_positive());
        error * self.tune.kp + derivative * self.tune.kd + self.summation * self.tune.ki
    }
}
pub struct EPID {
    pid: PID,
    output_mod: f32,
}
impl EPID {
    pub fn new(tune: PidTune) -> Self {
        Self {
            pid: PID::new(tune),
            output_mod: 1.0,
        }
    }
    pub fn update(&mut self, error: f32) -> f32 {
        match self.pid.prev_error_sign {
            None => {}
            Some(val) => {
                if (error).is_sign_positive() != val {
                    self.output_mod *= 0.5
                }
            }
        }
        self.pid.update(error) * self.output_mod
    }
}

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
