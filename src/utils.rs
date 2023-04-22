pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_add_numbers() {
    assert_eq!(add_numbers(2, 3), 5);
    assert_eq!(add_numbers(10,-5), 5);
}