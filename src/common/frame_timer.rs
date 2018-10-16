use std::time::{ Instant, Duration };

pub struct FrameTimer {
    timer: Instant,
    elapsed_time: Duration,
    calc_time: Duration,
    time_to_sleep: Duration
}

impl FrameTimer {
    const FRAME_TIME: Duration = Duration::from_nanos(1_000_000_000u64 / 60);

    pub fn new() -> Self {
        FrameTimer {
            timer: Instant::now(),
            elapsed_time: Duration::from_nanos(0),
            calc_time: Duration::from_nanos(0),
            time_to_sleep: Duration::from_nanos(0)
        }
    }

    pub fn elapsed_time(&self) -> Duration { self.elapsed_time }

    pub fn update(&mut self) {
        self.elapsed_time = self.timer.elapsed() + self.calc_time;
        let s = Instant::now();

        self.time_to_sleep = Duration::from_nanos(0);
        if Self::FRAME_TIME > self.elapsed_time {
            self.time_to_sleep = Self::FRAME_TIME - self.elapsed_time;
        }

        // self.print();

        ::std::thread::sleep(self.time_to_sleep);

        self.calc_time = s.elapsed() - self.time_to_sleep;
        self.elapsed_time = self.timer.elapsed();
        self.timer = Instant::now();
    }

    fn print(&mut self) {
        println!("elapsed: {:?} | sleep: {:?} | calc: {:?} | sum: {:?}", self.elapsed_time, self.time_to_sleep, self.calc_time, self.elapsed_time + self.time_to_sleep);
    }
}