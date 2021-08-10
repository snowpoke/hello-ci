fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::eq_op)]
    fn basic_test() {
        assert_eq!(2 + 2, 4);
    }
}
