use crate::prelude::*;
use std::{
    fs::{self, File},
    io::Write,
};

const CHUNK_DIR: &str = "savegame/chunks";
const WORLD_DIR: &str = "savegame/worlds";

#[derive(Debug)]
pub enum IOError {
    //Load
    FailedToOpenFile,
    FailedToReadFile,
    FailedToDecompressFile,
    FailedToDeserialize,

    // Save
    FailedToCreateDir,
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

pub fn chunk_save_location(chunk_file_name: &str) -> String {
    format!("{}/{}", CHUNK_DIR, chunk_file_name)
}

pub fn world_save_location(planet_file: &str) -> String {
    format!("{}/{}", WORLD_DIR, planet_file)
}

//////////////////////////////////////////////////////////////////////////////////////////
// IO Checks
//////////////////////////////////////////////////////////////////////////////////////////

pub fn check_file_exists(filename: &str) -> bool { std::path::Path::new(filename).exists() }

pub fn does_chunk_file_exist(chunk_id: ChunkLocation) -> bool {
    check_file_exists(&chunk_filename(chunk_id))
}

pub fn does_world_file_exist() -> bool { check_file_exists(&world_save_location("world.dat")) }

//////////////////////////////////////////////////////////////////////////////////////////
// IO Operations
//////////////////////////////////////////////////////////////////////////////////////////

pub fn load_data<D: for<'a> Deserialize<'a>>(file_path: String) -> Result<D, IOError> {
    use std::io::Read;
    use std::path::Path;

    if !check_file_exists(&file_path) {
        return Err(IOError::SaveFileDoesNotExist);
    }

    let loadpath = Path::new(&file_path);
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

pub fn setup_io_access() -> Result<(), IOError> {
    unwrap_or_return!(fs::create_dir_all(CHUNK_DIR), IOError::FailedToCreateDir);
    unwrap_or_return!(fs::create_dir_all(WORLD_DIR), IOError::FailedToCreateDir);

    Ok(())
}
