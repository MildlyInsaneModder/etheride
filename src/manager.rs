/// Configuration parameters for an exit-condition [`Manager`].
#[derive(Debug)]
pub struct ManagerParams {
    /// Absolute error threshold for the "small" (tight) settle band.
    small_settle_range: f32,
    /// Milliseconds the error must remain within `small_settle_range` before exiting.
    small_settle_time: u32,
    /// Absolute error threshold for the "large" (loose) settle band.
    large_settle_range: f32,
    /// Milliseconds the error must remain within `large_settle_range` before exiting.
    large_settle_time: u32,
    /// Maximum milliseconds to run before exiting unconditionally.
    timeout: u32,
}

impl ManagerParams {
    /// Create a new `ManagerParams`.
    ///
    /// * `small_settle_range` – tight error band (e.g. `16.0` degrees)
    /// * `small_settle_time` – ms to hold the tight band (e.g. `20`)
    /// * `large_settle_range` – loose error band (e.g. `40.0` degrees)
    /// * `large_settle_time` – ms to hold the loose band (e.g. `80`)
    /// * `timeout` – hard time limit in ms (e.g. `2000`)
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

/// Exit-condition manager for motion control loops.
///
/// Call [`Manager::update`] with the current error each iteration and check
/// [`Manager::should_exit`] to know when the motion is complete. The loop
/// exits when *any* of the following conditions is satisfied:
///
/// * The error has been within `small_settle_range` for at least
///   `small_settle_time` milliseconds.
/// * The error has been within `large_settle_range` for at least
///   `large_settle_time` milliseconds.
/// * The total elapsed time exceeds `timeout` milliseconds.
#[derive(Debug)]
pub struct Manager {
    params: ManagerParams,
    init_time: vexide::time::LowResolutionTime,
    small_settled_time: u32,
    large_settled_time: u32,
    prev_time: vexide::time::LowResolutionTime,
}

impl Manager {
    /// Create a new `Manager` with the given parameters. The timeout clock
    /// starts immediately on construction.
    pub fn new(params: ManagerParams) -> Self {
        Self {
            params,
            init_time: vexide::time::LowResolutionTime::now(),
            small_settled_time: 0,
            large_settled_time: 0,
            prev_time: vexide::time::LowResolutionTime::now(),
        }
    }

    /// Returns `true` when the motion should stop.
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

    /// Update internal settle timers with the current error value.
    ///
    /// Call this once per control-loop iteration *after* driving the motor.
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
