use std::time::{Duration, Instant};

pub struct Time {
    last_frame: Instant,
    delta_time: Duration,
    start_time: Instant,
}

impl Time {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            last_frame: now,
            delta_time: Duration::ZERO,
            start_time: now,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_frame);
        self.last_frame = now;
    }

    pub fn delta(&self) -> Duration {
        self.delta_time
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}

#[cfg(test)]
mod tests {
    use super::Time;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn time_increases() {
        let mut time = Time::new();

        let sleep_1 = Duration::from_millis(20);
        let sleep_2 = Duration::from_millis(30);

        sleep(sleep_1);
        time.update();
        let delta_1 = time.delta();
        let elapsed_1 = time.elapsed();

        sleep(sleep_2);
        time.update();
        let delta_2 = time.delta();
        let elapsed_2 = time.elapsed();

        assert!(delta_1 >= sleep_1);
        assert!(delta_2 >= sleep_2);

        assert!(elapsed_1 >= sleep_1);
        assert!(elapsed_2 >= sleep_1 + sleep_2);
    }
}
