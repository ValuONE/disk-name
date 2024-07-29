use disk_name::get_letters;

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::{Duration, SystemTime};

    use rand::{Rng, thread_rng};

    use crate::get_letters;

    #[test]
    #[cfg_attr(not(target_os = "windows"), ignore)]
    /// Run [`get_letters()`] 100 times to calculate the median speed
    fn get_median_runtime() {
        let times = 100;
        let mut sum = 0;
        for _ in 0..times {
            let t = SystemTime::now();
            let _ = get_letters();
            sum = sum + t.elapsed().unwrap().as_micros();

            // Sleep random period to simulate realistic method calls
            sleep(Duration::from_micros(thread_rng().gen_range(5..=5000)));
        }

        println!("Executed {} times with median of {:?}µs", times, sum / times);
    }
}