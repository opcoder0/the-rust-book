pub struct Guess {
    n: i32,
}

impl Guess {
    pub fn new(i: i32) -> Self {
        if i < 1 {
            panic!("value should be more than 1");
        } else if i > 100 {
            panic!("value should be less than 100");
        } else {
            Guess { n: i }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn generic_should_panic() {
        let _g = Guess::new(-1);
    }

    #[test]
    #[should_panic]
    fn should_panic_but_wont() {
        let _g = Guess::new(86);
    }

    #[test]
    #[should_panic(expected = "should be more than 1")]
    fn should_panic_less_than_lowerbound() {
        let _g = Guess::new(-1);
    }

    #[test]
    #[should_panic(expected = "should be less than 100")]
    fn should_panic_more_than_upperbound() {
        let _g = Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "value cannot be zero")]
    fn should_panic_wrong_expected_message() {
        let _g = Guess::new(-1);
    }
}
