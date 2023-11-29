fn main() {
    let upper_bound = 301;
    (0..upper_bound)
        .map(|i| {println!("{}", i); i})
        .map(|i| fizzbuzz_static_lookup_table(i))
        .filter(|s| s.is_some())
        .for_each(|s| println!("{}", s.unwrap()));

    println!("fizz buzz was seen {} times", upper_bound / 15 + 1);
}

pub fn fizzbuzz_match(i: i32) -> Option<&'static str> {
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

pub fn fizzbuzz_static_lookup_table(i: i32) -> Option<&'static str> {
    FIZZBUZZ_LOOKUP_TABLE[i as usize % 15 ]
}

pub fn fizz_buzz_match_2(i: i32) -> Option<&'static str> {
    match i {
        y if y % 3 == 0 && y % 5 == 0 => Some("fizz buzz"),
        y if y % 3 == 0 => Some("fizz"),
        y if y % 5 == 0 => Some("buzz"),
        _ => None
    }
}


pub fn fizz_buzz_lookup(x: i32) -> Option<&'static str> {
    let fizzbuzz_lookup_table = [
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

    fizzbuzz_lookup_table[x as usize % fizzbuzz_lookup_table.len()]
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
    fn test_fizzbuzz_match() {
        test_fizzbuzz_logic(fizzbuzz_match);
    }

    #[test]
    fn test_fizzbuzz_static_lookup_table() {
        test_fizzbuzz_logic(fizzbuzz_static_lookup_table);
    }

    #[test]
    fn test_fizzbuzz_lookup_table() {
        test_fizzbuzz_logic(fizz_buzz_lookup);
    }
    #[test]
    fn test_fizzbuzz_match_2() {
        test_fizzbuzz_logic(fizz_buzz_match_2);
    }

}
