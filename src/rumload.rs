use std::convert::TryInto;

/// Read in a str which is a file name of a program, read in its bits
/// turn the bits into words. Store the words in a vec and returns the vec
/// of u32 words
/// 
/// # Arguments:
/// * `str`: memory that is being manipulated
pub fn load(input: Option<&str>) -> Vec<u32> {
    let mut raw_reader: Box<dyn std::io::BufRead> = match input {
        None => Box::new(std::io::BufReader::new(std::io::stdin())),
        Some(filename) => Box::new(std::io::BufReader::new(
            std::fs::File::open(filename).unwrap(),
        )),
    };

    let mut buf = Vec::<u8>::new();

    raw_reader.read_to_end(&mut buf).unwrap();
    let instructions: Vec<u32> = buf
        .chunks_exact(4)
        .map(|x| u32::from_be_bytes(x.try_into().unwrap()))
        .collect();
    instructions
}
