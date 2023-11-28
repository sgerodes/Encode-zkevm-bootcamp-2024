fn main() {
    let upper_bound = 301;
    (0..upper_bound)
        .map(|i| {println!("{}", i); i})
        .map(|i| fizzbuzz3(i))
        .filter(|s| s.is_some())
        .for_each(|s| println!("{}", s.unwrap()));

    println!("fizz buzz was seen {} times", upper_bound / 15 + 1);
}

fn fizzbuzz(i: i32) -> Option<&'static str> {
    match (i % 3 == 0, i % 5 == 0) {
        (true, false) => Some("fizz"),
        (false, true) => Some("buzz"),
        (true, true) => Some("fizz buzz"),
        _ => None
    }
}

static FIZZBUZZ_LOOKUP_TABLE: [Option<&'static str>; 15] = [
    Some("fizz buzz"), // 0
    None,              // 1
    None,              // 2
    Some("fizz"),      // 3
    None,              // 4
    Some("buzz"),      // 5
    Some("fizz"),      // 6
    None,              // 7
    None,              // 8
    Some("fizz"),      // 9
    Some("buzz"),      // 10
    None,              // 11
    Some("fizz"),      // 12
    None,              // 13
    None,              // 14
];

fn fizzbuzz3(i: i32) -> Option<&'static str> {
    FIZZBUZZ_LOOKUP_TABLE[i as usize % 15 ]
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_fizzbuzz_logic<F>(fizzbuzz_func: F)
        where
            F: Fn(i32) -> Option<&'static str>,
    {
        assert_eq!(fizzbuzz_func(3).map(|s| s.to_string()), Some("fizz".to_string()));
        assert_eq!(fizzbuzz_func(5).map(|s| s.to_string()), Some("buzz".to_string()));
        assert_eq!(fizzbuzz_func(15).map(|s| s.to_string()), Some("fizz buzz".to_string()));
        assert_eq!(fizzbuzz_func(1), None);
        assert_eq!(fizzbuzz_func(30).map(|s| s.to_string()), Some("fizz buzz".to_string()));
        assert_eq!(fizzbuzz_func(97), None);
    }

    #[test]
    fn test_fizzbuzz() {
        test_fizzbuzz_logic(fizzbuzz);
    }

    #[test]
    fn test_fizzbuzz2() {
        test_fizzbuzz_logic(fizzbuzz2);
    }

    #[test]
    fn test_fizzbuzz3() {
        test_fizzbuzz_logic(fizzbuzz3);
    }
}
