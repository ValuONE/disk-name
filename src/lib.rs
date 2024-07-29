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
