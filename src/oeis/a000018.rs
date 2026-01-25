/// Number of positive integers <= 2^n not divisible by 5.
/// https://oeis.org/A000018

pub struct A000018;

impl crate::traits::IntegerSequence for A000018 {
    const NAME: &str = "Number of positive integers <= 2^n not divisible by 5";

    const HEAD: &[crate::Value] = &[
        1, 2, 4, 7, 13, 26, 52, 103, 205, 410, 820, 1639, 3277, 6554, 13108, 26215, 52429, 104858,
        209716, 419431, 838861, 1677722, 3355444, 6710887, 13421773, 26843546, 53687092, 107374183,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000018";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        if n < 0 {
            return 0;
        }
        let pow2n = 1isize << n;
        pow2n - pow2n / 5
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000018>();
}
