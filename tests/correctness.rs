use std::collections::HashMap;
use tailcall::*;

fn factorial(input: u64) -> u64 {
    #[tailcall]
    fn factorial_inner(accumulator: u64, input: u64) -> u64 {
        if input > 0 {
            factorial_inner(accumulator * input, input - 1)
        } else {
            accumulator
        }
    }

    factorial_inner(1, input)
}

#[test]
fn test_factorial_correctness() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(4), 24);
}

fn memoized_factorial(input: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    #[tailcall]
    fn factorial_inner(accumulator: u64, input: u64, memo: &mut HashMap<u64, u64>) -> u64 {
        memo.insert(input, accumulator);

        if input > 0 {
            factorial_inner(accumulator * input, input - 1, memo)
        } else {
            accumulator
        }
    }

    factorial_inner(1, input, memo)
}

#[test]
fn test_memoized_factorial_correctness() {
    let mut memo = HashMap::new();

    assert_eq!(memoized_factorial(4, &mut memo), 24);
    assert_eq!(memo.get(&0), Some(&24));
    assert_eq!(memo.get(&1), Some(&24));
    assert_eq!(memo.get(&2), Some(&12));
    assert_eq!(memo.get(&3), Some(&4));
    assert_eq!(memo.get(&4), Some(&1));
}
