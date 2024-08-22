use disk_name::get_letters;

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::{Duration, SystemTime};

    use crate::get_letters;
    use rand::{thread_rng, Rng};
    use winapi::um::fileapi::{GetVolumeInformationA};
    use winapi::um::winbase::{GetLogicalDriveStringsA, OFS_MAXPATHNAME};

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

        println!("RustSTD: Executed {} times with median of {:?}µs", times, sum / times);

        let times = 100;
        let mut sum = 0;
        unsafe {
            for _ in 0..times {
                let t = SystemTime::now();
                let _ = get_letter_with_win();
                sum = sum + t.elapsed().unwrap().as_micros();

                // Sleep random period to simulate realistic method calls
                sleep(Duration::from_micros(thread_rng().gen_range(5..=5000)));
            }
        }

        println!("WinAPI: Executed {} times with median of {:?}µs", times, sum / times);
    }

    #[cfg_attr(not(target_os = "windows"), ignore)]
    unsafe fn get_letter_with_win() {
        let mut drive_names = [0u8; OFS_MAXPATHNAME];
        let drive_name_len = GetLogicalDriveStringsA(OFS_MAXPATHNAME as u32, drive_names.as_mut_ptr() as *mut i8);

        if drive_name_len > 0 {
            let mut drive_name_ptr = drive_names.as_ptr();
            while drive_name_ptr < drive_names.as_ptr().add(drive_name_len as usize) {
                let mut volume_name = [0u8; OFS_MAXPATHNAME];
                let mut volume_serial_number = 0;
                let mut max_component_name_length = 0;
                let mut file_system_flags = 0;
                let mut max_filename_length = 0;

                GetVolumeInformationA(
                    drive_name_ptr as *mut i8,
                    volume_name.as_mut_ptr() as *mut i8,
                    OFS_MAXPATHNAME as u32,
                    &mut volume_serial_number,
                    &mut max_component_name_length,
                    &mut file_system_flags,
                    &mut max_filename_length,
                    0,
                );

                drive_name_ptr = drive_name_ptr.add(2);
            }
        }
    }
}