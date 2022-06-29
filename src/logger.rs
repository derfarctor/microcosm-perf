use colour::e_green;
use std::sync::{atomic, atomic::AtomicUsize, Arc, Mutex};
use std::{thread, time};
type Terminator = Arc<Mutex<bool>>;
type CombinationsTested = Arc<AtomicUsize>;

pub fn threaded_logger(
    log_combinations: CombinationsTested,
    terminated: Terminator,
    complexity: u64,
) {
    let start_time = time::Instant::now();
    thread::sleep(time::Duration::from_millis(5000));
    loop {
        let tested = log_combinations.load(atomic::Ordering::Relaxed);
        let percentage = 100. * (tested as f64 / complexity as f64);
        let runtime = start_time.elapsed();
        let per_second =
            (tested as f64) / (runtime.as_secs() as f64 + runtime.subsec_millis() as f64 / 1000.0);
        e_green!(
            "\r{:>width$}\r{:.2}% done | {} combinations per second",
            "",
            percentage,
            per_second as u64,
            width = 80
        );
        thread::sleep(time::Duration::from_millis(1000));
        if *terminated.lock().unwrap() {
            break;
        }
    }
}
