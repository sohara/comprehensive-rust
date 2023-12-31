// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    // 1. Remove spaces
    // 2. Loop over string chars backwards, doubling every 2nd
    //      - If the value is greater than 9, sum the digits
    // 3. Sum all the digits
    // return whether sum ends in 0
    let mut sanitized = String::new();
    for c in cc_number.chars() {
        if !c.is_whitespace() {
            sanitized.push(c);
        }
    }
    if sanitized.len() < 2 {
        return false;
    }
    let mut processed_digits = String::new();
    for (i, c) in sanitized.chars().rev().enumerate() {
        let digit = match c.to_digit(10) {
            Some(d) => d,
            None => return false,
        };

        let value = if i % 2 == 0 {
            digit
        } else {
            let doubled = digit * 2;
            if doubled > 9 {
                doubled - 9
            } else {
                doubled
            }
        };

        processed_digits.push(std::char::from_digit(value, 10).unwrap());
    }
    let mut summed = 0;
    for c in processed_digits.chars() {
        summed += c.to_digit(10).unwrap();
    }
    let last_char_summed = summed.to_string().chars().last().unwrap().to_string();

    last_char_summed == "0"
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}

