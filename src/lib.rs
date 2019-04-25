#[macro_export]
macro_rules! s {
    ($e:literal) => {$e.to_owned() as String};
}

#[macro_export]
macro_rules! string {
    ($e:literal) => {$e.to_owned() as String};
}

#[cfg(test)]
mod test {
    // enforces same type
    fn strict_equals<T: std::cmp::PartialEq>(a: T, b: T) -> bool {
        return a == b;
    }

    #[test]
    fn same_type_and_value() {
        assert!(strict_equals(s!("abcd"), "abcd".to_owned()));
        assert!(strict_equals(string!("abcd"), "abcd".to_owned()));
        println!("OK");
    }
}
