pub struct FpsCounter {
    begin_time: f64,
    prev_time: f64,
    frames: usize,
    pub current: f64,
}

impl FpsCounter {
    pub fn new() -> Self {}

    pub fn now() -> f64 {}

    pub fn begin(&mut self) {}

    pub fn end(&mut self) {}
}