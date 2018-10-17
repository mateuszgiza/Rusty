use std::time::{ Instant, Duration };

pub struct FrameTimer {
    pub is_sleep_enabled: bool,
    timer: Instant,
    elapsed_time: Duration,
    calc_time: Duration,
    time_to_sleep: Duration
}

impl FrameTimer {
    const MAX_FRAMES: u64 = 60;
    const FRAME_TIME: Duration = Duration::from_nanos(1_000_000_000u64 / Self::MAX_FRAMES);

    pub fn new() -> Self {
        FrameTimer {
            is_sleep_enabled: true,
            timer: Instant::now(),
            elapsed_time: Duration::from_nanos(0),
            calc_time: Duration::from_nanos(0),
            time_to_sleep: Duration::from_nanos(0)
        }
    }

    pub fn elapsed_time(&self) -> Duration { self.elapsed_time }

    pub fn update(&mut self) {
        let current_frame_time = self.timer.elapsed() + self.calc_time;
        let calculation_timer = Self::begin_measuring_calculation();

        self.time_to_sleep = Self::calculate_time_to_sleep(current_frame_time);
        
        if self.is_sleep_enabled {
            self.execute_sleep();
        }
        
        self.update_times(calculation_timer);        
    }

    fn begin_measuring_calculation() -> Instant {
        return Instant::now();
    }

    fn calculate_time_to_sleep(current_frame_time: Duration) -> Duration {
        if Self::FRAME_TIME > current_frame_time {
            return Self::FRAME_TIME - current_frame_time;
        }
        return Duration::from_nanos(0);
    }

    fn execute_sleep(&self) {
        ::std::thread::sleep(self.time_to_sleep);
    }

    fn update_times(&mut self, calculation_timer: Instant) {
        if self.is_sleep_enabled {
            self.calc_time = calculation_timer.elapsed() - self.time_to_sleep;
        } else {
            self.calc_time = calculation_timer.elapsed();
        }

        self.elapsed_time = self.timer.elapsed();
        self.timer = Instant::now();
    }
}