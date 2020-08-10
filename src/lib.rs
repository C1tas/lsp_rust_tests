pub fn add_2(num: i32) -> i32 {
    num + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn add_2_test() {
        assert_eq!(4, add_2(2));
    }
}
