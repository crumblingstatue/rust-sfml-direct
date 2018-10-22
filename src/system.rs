use crate::raw::*;

#[derive(Debug)]
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
    pub fn restart(&mut self) -> Time {
        Time(unsafe { self.0.restart() })
    }
}

#[derive(Debug)]
pub struct Time(sf_Time);

impl Default for Time {
    fn default() -> Self {
        Time(unsafe { sf_Time::new() })
    }
}

impl Time {
    pub fn as_seconds(&self) -> f32 {
        unsafe { self.0.asSeconds() }
    }
    pub fn as_milliseconds(&self) -> i32 {
        unsafe { self.0.asMilliseconds() }
    }
    pub fn as_microseconds(&self) -> i64 {
        unsafe { self.0.asMicroseconds() }
    }
}
