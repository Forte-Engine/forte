use std::{fs::File, io::Read};

/// A struct to contain functions for loading files.
pub struct Files;
impl Files {
    /// A function to load the bytes of a file at the given relative path.
    /// 
    /// Arguments:
    /// * path: &str - the relative path to the file.
    /// 
    /// Returns a result:
    /// * Ok - Vec<u8> - The bytes of the file.
    /// * Error - std::io::Error - The io error that occured while failing to load the file.
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