use sfml_direct::system::{sleep, Clock, Time};

fn main() {
    println!("Default time is {:?}", Time::default());
    let mut clock = Clock::default();
    for i in 0.. {
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
        match i % 3 {
            0 => {
                println!("Sleeping 1.2 seconds");
                sleep(Time::seconds(1.2));
            }
            1 => {
                println!("Sleeping 500 milliseconds");
                sleep(Time::milliseconds(500));
            }
            2 => {
                println!("Sleeping 500 microseconds");
                sleep(Time::microseconds(500));
            }
            _ => {}
        }
    }
}
