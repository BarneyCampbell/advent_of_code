use std::str::FromStr;

#[derive(Debug)]
pub enum ReadError {
    ReadLineError,
    ReadChunkError,
    ReadEntireError
}

#[derive(Debug)]
pub enum AOCError {
    ReadError(ReadError),
    ParseError
}

pub fn read_lines<T>(filepath: &str) -> Result<Vec<T>, AOCError>
    where T : FromStr + Default {
    if let Ok(filestr) = std::fs::read_to_string(filepath) {
        Ok(filestr.lines()
               .map(|line| line.parse::<T>().unwrap_or_default())
               .collect())
    }
    else {
        Err(AOCError::ReadError(ReadError::ReadLineError))
    }
}

pub fn read_lines_optional<T>(filepath: &str) -> Result<Vec<Option<T>>, AOCError>
    where T : FromStr {
    if let Ok(filestr) = std::fs::read_to_string(filepath) {
        Ok(filestr.lines()
            .map(|line| line.parse::<T>().ok())
            .collect())
    }
    else {
        Err(AOCError::ReadError(ReadError::ReadLineError))
    }
}

pub fn read_entire<T>(filepath: &str) -> Result<T, AOCError> 
    where T : FromStr {
    if let Ok(filestr) = std::fs::read_to_string(filepath) {
        match filestr.parse::<T>() {
            Ok(file) => Ok(file),
            Err(_)   => Err(AOCError::ParseError)
        }
    }
    else { Err(AOCError::ReadError(ReadError::ReadEntireError)) }
}
