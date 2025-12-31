use std::time::Instant;

pub struct Time {
    last_frame: Instant,
    delta_time: f32,
    start_time: Instant,
}

impl Time {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            last_frame: now,
            delta_time: 0.0,
            start_time: now,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame);
        self.delta_time = delta.as_secs_f32();
        self.last_frame = now;
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn elapsed_time(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn time_starts_with_zero_delta() {
        let time = Time::new();

        assert_eq!(time.delta_time(), 0.0);
        assert!(time.elapsed_time() >= 0.0);
    }

    #[test]
    fn update_increases_delta_time() {
        let mut time = Time::new();

        sleep(Duration::from_millis(1));
        time.update();

        assert!(time.delta_time() > 0.0);
        assert!(time.elapsed_time() > 0.0);
    }
}