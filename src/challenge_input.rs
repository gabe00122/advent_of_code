use std::fs;
use std::io;
use std::path::Path;

pub fn get(base_path: &str, year: u16, day: u8) -> io::Result<String> {
    let year_directory_name = format!("year{}", year);
    let file_name = format!("day{}.txt", day);

    let path = Path::new(base_path)
        .join(&year_directory_name)
        .join(&file_name);

    fs::read_to_string(&path)
}
