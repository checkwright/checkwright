pub fn doubled(n: i32) -> i32 {
    n * 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn doubling_is_doubling() {
        assert_eq!(super::doubled(21), 42);
    }
}
