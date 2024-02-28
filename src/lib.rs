pub fn version() -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = version();
        assert_eq!(result, 1);
    }
}
