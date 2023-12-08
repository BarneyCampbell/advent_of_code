use std::str::FromStr;

#[derive(Debug)]
pub enum ReadError {
    ReadLineError,
    ReadChunkError,
    ReadEntireError,
    ReadByteVecError,
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

pub fn read_char_vec(filepath: &str) -> Result<Vec<Vec<char>>, AOCError> {
    if let Ok(filestr) = std::fs::read_to_string(filepath) {
        let mut res: Vec<Vec<char>> = Vec::new();

        for line in filestr.lines() {
            res.push(line.chars().collect::<Vec<char>>());
        }

        Ok(res)
    }
    else { Err(AOCError::ReadError(ReadError::ReadByteVecError)) }
}

pub fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}

pub mod maths {
    pub fn lcm(v: Vec<u64>) -> u64 {
        if v.len() == 1 {
            return v[0];
        }
        if v.len() == 2 {
            return lcm_pair(v[0], v[1]);
        }
    
        let (x, xs) = v.split_at(2); // #Haskell
        return lcm_pair(lcm(x.to_vec()), lcm(xs.to_vec()));
    }
    
    fn lcm_pair(a: u64, b: u64) -> u64 {
        let numbers = vec![a, b];
        let mut temp = numbers.clone();
        
        // check all the same
        loop {
            let mut same = true;
    
            for idx in 1..temp.len() {
                if temp[0] != temp[idx] {
                    same = false;
                    break;
                }
            }
    
            if same {
                return temp[0];
            }
    
            // Find lowest index
            match temp.iter().enumerate().min_by(|(_, a), (_, b)| a.cmp(b)).map(|(index, _)| index) {
                Some(idx) => {
                    temp[idx] = temp[idx] + numbers[idx];
                },
                None => panic!("Not possible")
            }
        }
    }
}
