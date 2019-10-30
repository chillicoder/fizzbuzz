fn main() {
    for _ix in 1..101 {
        println!("{}", fizzbuzz(_ix));
    }
}

fn fizzbuzz(value: usize) -> String {

    unimplemented!();
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz_1() {
        assert!("1" == fizzbuzz(1));
    }

    #[test]
    fn test_fizzbuzz_2() {
        assert_eq!("2", fizzbuzz(2), "Should be '2'");
    }

   #[test]
    fn test_fizzbuzz_3() {
        assert_eq!("Fizz", fizzbuzz(3));
    }

    #[test]
    fn test_fizzbuzz_5() {
        assert_eq!("Buzz", fizzbuzz(5));
    }

    #[test]
    fn test_fizzbuzz_15() {
        assert_eq!("FizzBuzz", fizzbuzz(15));
    }
}

