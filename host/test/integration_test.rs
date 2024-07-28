pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
fn test_add() {
    #[test]
    let result = add(2, 2);
    assert_eq!(result, 4);
}