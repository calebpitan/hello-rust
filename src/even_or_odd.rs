pub fn is_even(num: usize) -> bool {
    if num % 2 == 0 {true} else {false}
}

pub fn is_odd(num: usize) -> bool {
    !is_even(num)
}
