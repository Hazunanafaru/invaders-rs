use std::time::Duration;

use rusty_time::timer::Timer;

use crate::frame::Drawable;

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

impl Shot {
    // Initialize a new shot
    pub fn new(x: usize, y: usize) -> Self {
        Self { x: x, y: y, exploding: false, timer: Timer::from_millis(50) }
    }

    // Update the shot duration
    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }

    // Explode the shot
    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(300);
    }

    // Shot is dead
    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || (self.y == 0)
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x][self.y] = if self.exploding { "*" } else { "|" };
    }
}