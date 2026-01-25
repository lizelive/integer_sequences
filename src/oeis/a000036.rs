/// High water marks for number of divisors function: a(n) is the smallest number whose number of divisors is higher than that of any smaller number.
/// https://oeis.org/A000036

pub struct A000036;

impl crate::traits::IntegerSequence for A000036 {
    const NAME: &str = "High water marks for number of divisors function";

    const HEAD: &[crate::Value] = &[
        1, 2, 4, 6, 12, 24, 36, 48, 60, 120, 180, 240, 360, 720, 840, 1260, 1680, 2520, 5040, 7560,
        10080, 15120, 20160, 25200, 27720, 45360, 50400, 55440, 83160, 110880, 166320, 221760,
        277200, 332640, 498960, 554400, 665280,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000036";

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
    crate::tester::test_sequance_formula_matchces_head::<A000036>();
}
