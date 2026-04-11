struct Odom {
    vertical_offset: f32,
    horizontal_offset: f32,
    y_position: f32,
    x_position: f32,
}
impl Odom {
    pub fn new(vertical_offset: f32, horizontal_offset: f32) -> Self {
        Self {
            vertical_offset,
            horizontal_offset,
            y_position: 0.0,
            x_position: 0.0,
        }
    }
}
