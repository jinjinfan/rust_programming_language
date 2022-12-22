pub fn add(left: usize, right: usize) -> usize {
    if left < 1 {
        panic!("Left should be greater or equal to 1")
    } else {
        left + right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert!(result == 4);
    }
    #[test]
    #[should_panic(expect = "Left should be greater or equal to 1")]
    fn less_than_1() {
        add(0, 2);
    }
    
    #[test]
    fn it_works2()-> Result<(),String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
