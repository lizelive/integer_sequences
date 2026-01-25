/// Number of partitions of n into parts each of which is used a different number of times.
/// https://oeis.org/A000058

pub struct A000058;

impl crate::traits::IntegerSequence for A000058 {
    const NAME: &str = "Sylvester's sequence: a(n+1) = a(n)^2 - a(n) + 1";

    const HEAD: &[crate::Value] = &[
        2, 3, 7, 43, 1807, 3263443,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000058";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        if n < 0 {
            return 0;
        }
        // Sylvester's sequence: a(0)=2, a(n+1) = a(n)^2 - a(n) + 1
        let mut result = 2isize;
        let mut i = 0;
        while i < n {
            result = result * result - result + 1;
            i += 1;
        }
        result
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000058>();
}
