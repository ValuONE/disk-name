use std::path::Path;

/// Fetches all existing hard drive letters
///
/// Returns a [`Vec<String>`]
pub fn get_letters() -> Vec<String> {
    HARD_DRIVE_NAMES
        .iter()
        .filter(|name| Path::new(name).exists())
        .map(|name| name.to_string())
        .collect()
}

/// Static array of all possible drive letters from A-Z
static HARD_DRIVE_NAMES: [&str; 26] = [
    "A:\\", "B:\\", "C:\\", "D:\\", "E:\\",
    "F:\\", "G:\\", "H:\\", "I:\\", "J:\\",
    "K:\\", "L:\\", "M:\\", "N:\\", "O:\\",
    "P:\\", "Q:\\", "R:\\", "S:\\", "T:\\",
    "U:\\", "V:\\", "W:\\", "X:\\", "Y:\\",
    "Z:\\",
];

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::{Duration, SystemTime};
    use rand::{Rng, thread_rng};
    use crate::get_letters;

    #[test]
    #[cfg_attr(not(target_os = "windows"), ignore)]
    /// Windows specific test to check for any drive
    fn check_for_any_drive() {
        let t = SystemTime::now();
        let letters = get_letters();
        let duration = t.elapsed().unwrap_or_default();
        println!("Time elapsed: {:?}, letters found: {:#?}", duration, letters);

        assert!(letters.len() >= 1);
    }

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