#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use disk_name::get_letters;

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
}