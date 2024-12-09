// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    // The `todo!()` macro is a placeholder that the compiler
    // interprets as "I'll get back to this later", thus
    // suppressing type errors.
    // It panics at runtime.

    // result = 1, 
    // m = 5, result = 1 * 5 (5)
    // m = 4, result = 5 * 4 (20)
    // m = 3, result = 20 * 3 (60)
    // m = 2, result = 60 * 2 (120)
    //

    let mut result = 1;
    let mut m = n;

    while m > 1 {
        result = result * m;
        m = m - 1 ;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
