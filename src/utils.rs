pub fn wrap_180(mut input: f32) -> f32 {
    while -180.0 >= input || 180.0 < input {
        if input <= -180.0 {
            input += 360.0;
        }
        if input > 180.0 {
            input -= 360.0;
        }
    }
    input
}
