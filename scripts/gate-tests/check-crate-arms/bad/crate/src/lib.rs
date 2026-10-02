pub fn doubled(n: i32) -> i32 {
    let unread = n;
    n * 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn doubling_is_doubling() {
        assert_eq!(super::doubled(21), 43);
    }
}
