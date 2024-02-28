pub fn version() -> i32 {
    1
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
