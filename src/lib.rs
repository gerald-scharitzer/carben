use std::env;
use std::io::{Result, Stdout, Write};

pub fn version() -> i32 {
    1
}

pub struct MainRunner {
    args: Vec<String>,
    stdout: Stdout
}

impl MainRunner {
    pub fn new() -> MainRunner {
        MainRunner {
            args: env::args().collect(),
            stdout: std::io::stdout()
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let version = version();
        let result = writeln!(self.stdout, "version {version}");
        if result.is_err() {
            return result;
        }
        let argc = self.args.len();
        let result = writeln!(self.stdout, "argc {argc}");
        if result.is_err() {
            return result;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let result = version();
        assert_eq!(result, 1);
    }
}
