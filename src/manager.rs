#[derive(Debug)]
pub struct ManagerParams {
    small_settle_range: f32,
    small_settle_time: u32,
    large_settle_range: f32,
    large_settle_time: u32,
    timeout: u32,
}

impl ManagerParams {
    pub fn new(
        small_settle_range: f32,
        small_settle_time: u32,
        large_settle_range: f32,
        large_settle_time: u32,
        timeout: u32,
    ) -> Self {
        Self {
            small_settle_range,
            small_settle_time,
            large_settle_range,
            large_settle_time,
            timeout,
        }
    }
}
#[derive(Debug)]
pub struct Manager {
    params: ManagerParams,
    init_time: vexide::time::LowResolutionTime,
    small_settled_time: u32,
    large_settled_time: u32,
    prev_time: vexide::time::LowResolutionTime,
}

impl Manager {
    pub fn new(params: ManagerParams) -> Self {
        Self {
            params,
            init_time: vexide::time::LowResolutionTime::now(),
            small_settled_time: 0,
            large_settled_time: 0,
            prev_time: vexide::time::LowResolutionTime::now(),
        }
    }
    pub fn should_exit(&self) -> bool {
        if self.params.small_settle_time <= self.small_settled_time {
            return true;
        }
        if self.params.large_settle_time <= self.large_settled_time {
            return true;
        }
        if self.params.timeout
            <= vexide::time::LowResolutionTime::now()
                .duration_since(self.init_time)
                .as_millis() as u32
        {
            return true;
        }
        false
    }
    pub fn update(&mut self, error: f32) {
        let dt = vexide::time::LowResolutionTime::now()
            .duration_since(self.prev_time)
            .as_millis() as u32;
        if error.abs() <= self.params.small_settle_range {
            self.small_settled_time += dt;
        } else {
            self.small_settled_time = 0;
        }
        if error.abs() <= self.params.large_settle_range {
            self.large_settled_time += dt;
        } else {
            self.large_settled_time = 0;
        }
        self.prev_time = vexide::time::LowResolutionTime::now();
    }
}
