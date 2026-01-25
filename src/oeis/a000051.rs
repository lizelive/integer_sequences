/// Number of ways of choosing 4 integers that sum to n.
/// https://oeis.org/A000051

pub struct A000051;

impl crate::traits::IntegerSequence for A000051 {
    const NAME: &str = "a(n) = 2^n + 1";

    const HEAD: &[crate::Value] = &[
        2, 3, 5, 9, 17, 33, 65, 129, 257, 513, 1025, 2049, 4097, 8193, 16385, 32769, 65537, 131073,
        262145, 524289, 1048577, 2097153, 4194305, 8388609, 16777217, 33554433, 67108865, 134217729,
        268435457, 536870913, 1073741825, 2147483649, 4294967297,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000051";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        if n < 0 {
            return 0;
        }
        (1isize << n) + 1
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000051>();
}
