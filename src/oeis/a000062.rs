/// a(n) = Fibonacci(n-1) + Fibonacci(n+1).
/// https://oeis.org/A000062

pub struct A000062;

impl crate::traits::IntegerSequence for A000062 {
    const NAME: &str = "Primes p with primitive root 4";

    const HEAD: &[crate::Value] = &[
        3, 5, 7, 11, 13, 19, 23, 37, 41, 43, 47, 53, 59, 61, 67, 79, 83, 97, 101, 103, 107, 109,
        127, 131, 139, 149, 157, 163, 179, 181, 191, 197, 199, 211, 223, 227, 229, 239, 251, 263,
        269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 347, 349, 353, 359, 367, 373, 379,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000062";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        0
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000062>();
}
