use carben;

#[test]
fn test_version() {
    let result = carben::version();
    assert_eq!(result, 1);
}
