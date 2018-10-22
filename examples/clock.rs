use sfml_direct::system::{Clock, Time};

fn main() {
    println!("Default time is {:?}", Time::default());
    let mut clock = Clock::default();
    loop {
        let t = clock.elapsed_time();
        println!(
            "{} : {} : {}",
            t.as_seconds(),
            t.as_milliseconds(),
            t.as_microseconds()
        );
        if t.as_seconds() > 5.0 {
            println!("Restarting clock...");
            clock.restart();
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
}
