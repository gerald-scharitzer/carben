use std::fs::File;
use std::io::{BufReader, stdin};

pub fn config(filename: &str) -> Result<(), Box<dyn Error>> { // FIXME
    if filename == "-" {
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer)?; // TODO get from MainRunner
        let config = toml::from_str(&buffer)?;
    } else {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let config = toml::from_reader(reader)?;
    }
    Ok(())
}
