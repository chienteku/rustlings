fn join_string<'a>(a: &'a str, b:&'a str) -> String {
    format!("{}{}", a, b)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        assert_eq!(join_string("a", "b"), "ab");
    }
}
