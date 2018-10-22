use sfml_direct::system::Clock;

fn main() {
    let clock = Clock::default();
    let t = clock.elapsed_time();
    println!("{}", t.as_seconds());
}
