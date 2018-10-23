use std::time::Duration;
use specs::{ System, Read, WriteStorage };
use components::{ Text, FPS };
use sdl2_extras::common::GameTime;

pub struct FpsCounter {
    counter: u16,
    elapsed_time: Duration
}

impl FpsCounter {
    pub fn new() -> Self {
        FpsCounter {
            counter: 0,
            elapsed_time: Duration::from_nanos(0)
        }
    }
}

impl<'a> System<'a> for FpsCounter {
    type SystemData = (
        Read<'a, GameTime>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, FPS>
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (game_time, mut text, mut fps) = data;

        self.counter += 1;
        self.elapsed_time += game_time.delta.elapsed;

        for (text, fps) in (&mut text, &mut fps).join() {
            if self.elapsed_time >= fps.probe_time {
                fps.fps_count = self.counter;
                self.counter = 0;
                self.elapsed_time -= fps.probe_time;

                text.text = format!("FPS: {} | frame_time: {:?}ms", fps.fps_count, (&game_time.delta / 1_000_000.0 * 100.0).round() / 100.0);
            }
        }
    }
}