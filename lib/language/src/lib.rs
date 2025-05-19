use std::{
    fs::{self, File},
    io::Read,
};

pub fn import_db<P: AsRef<str>>(path: P) -> Vec<u8> {
    let path = path.as_ref();
    let mut file = File::open(path).expect("Failed to open file");
    let metadata = fs::metadata(&path).expect("unable to read metadata");

    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("Failed to read file");
    buffer
}
