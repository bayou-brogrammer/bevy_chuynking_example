use crate::prelude::*;
use std::{fs::File, io::Write};

#[derive(Debug)]
pub enum IOError {
    //Load
    FailedToOpenFile,
    FailedToReadFile,
    FailedToDecompressFile,
    FailedToDeserialize,

    // Save
    FailedToCreateFile,
    SaveFileDoesNotExist,
    SaveFileCorrupted,
    FailedToSerialize,
}

macro_rules! unwrap_or_return {
    ( $e:expr, $err:expr) => {
        match $e {
            Ok(x) => x,
            Err(_) => return Err($err),
        }
    };
}

pub fn load_data<D: for<'a> Deserialize<'a>>(file_path: String) -> Result<D, IOError> {
    use std::io::Read;
    use std::path::Path;

    let loadpath = Path::new(&file_path);
    if !loadpath.exists() {
        return Err(IOError::SaveFileDoesNotExist);
    }

    let mut f = unwrap_or_return!(File::open(loadpath), IOError::FailedToOpenFile);
    let mut buffer = Vec::<u8>::new();
    unwrap_or_return!(f.read_to_end(&mut buffer), IOError::FailedToReadFile);

    let raw_bytes = unwrap_or_return!(
        miniz_oxide::inflate::decompress_to_vec(&buffer),
        IOError::FailedToDecompressFile
    );

    Ok(unwrap_or_return!(bincode::deserialize(&raw_bytes), IOError::FailedToDeserialize))
}

pub fn save_data<D: Serialize>(file_path: String, data: D) -> Result<(), IOError> {
    let mut file = unwrap_or_return!(File::create(file_path), IOError::FailedToCreateFile);
    let mem_vec = unwrap_or_return!(bincode::serialize(&data), IOError::SaveFileCorrupted);
    let compressed_bytes = miniz_oxide::deflate::compress_to_vec(&mem_vec, 6);
    unwrap_or_return!(file.write_all(&compressed_bytes), IOError::FailedToSerialize);

    Ok(())
}
