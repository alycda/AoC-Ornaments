pub trait Solution {
    /// Ensures the output can be converted to a string
    type Output: std::fmt::Display;  
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Day;

    impl Day {
        fn solve() -> <Self as Solution>::Output {
            "Hello, Christmas!".to_string()
        }
    }

    impl Solution for Day {
        type Output = String;
    }

    #[test]
    fn it_works() {
        let solution = Day::solve().to_string();
        assert_eq!("Hello, Christmas!".to_string(), solution);
    }
}
