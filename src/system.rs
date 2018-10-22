use crate::raw::*;

pub struct Clock(sf_Clock);

impl Default for Clock {
    fn default() -> Self {
        Clock(unsafe { sf_Clock::new() })
    }
}

impl Clock {
    pub fn elapsed_time(&self) -> Time {
        Time(unsafe { self.0.getElapsedTime() })
    }
}

pub struct Time(sf_Time);

impl Time {
    pub fn as_seconds(&self) -> f32 {
        unsafe { self.0.asSeconds() }
    }
}
