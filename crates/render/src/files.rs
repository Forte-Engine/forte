use std::{fs::File, io::Read};

pub struct Files;
impl Files {
    pub fn load_bytes(path: &str) -> Result<Vec<u8>, std::io::Error> {
        // get file and metadata
        let mut file = File::open(&path)?;
        let metadata = file.metadata()?;

        // load file into a buffer
        let mut buffer = vec![0; metadata.len() as usize];
        let _ = file.read(&mut buffer);

        // return the buffer
        return Ok(buffer);
    }
}