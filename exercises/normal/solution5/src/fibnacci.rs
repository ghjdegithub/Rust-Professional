pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut fib1 = 0;
    let mut fib2 = 1;
    let mut sum = 0;

    while fib2 <= threshold {
        if fib2 % 2 != 0 {
            sum += fib2;
        }

        let next_fib = fib1 + fib2;
        fib1 = fib2;
        fib2 = next_fib;
    }

    sum
}
