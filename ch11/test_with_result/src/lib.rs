pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let r = 100;
        if add(50, 50) == r {
            Ok(())
        } else {
            Err(String::from("error in addition routine"))
        }
    }

    #[test]
    fn it_not_works() -> Result<(), String> {
        let r = 10;
        if add(50, 50) == r {
            Ok(())
        } else {
            Err(String::from("something is fishy"))
        }
    }
}
