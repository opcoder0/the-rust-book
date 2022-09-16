#[derive(Debug)]
pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Self {
        Self {
            width: width,
            height: height,
        }
    }

    fn fits_in(&self, r: &Rectangle) -> bool {
        self.width < r.width && self.height < r.height
    }
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub fn echo(s: &str) -> &str {
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn always_fail() {
        panic!("this test always fails");
    }

    #[test]
    fn test_echo() {
        let s = "echo";
        assert!(s == echo(s));
    }

    #[test]
    fn test_rectangle_fits() {
        let r1 = Rectangle::new(5, 10);
        let r2 = Rectangle::new(4, 8);
        assert!(r2.fits_in(&r1));
    }

    #[test]
    fn add_two_ok() {
        assert_eq!(4, add_two(2));
    }
}
