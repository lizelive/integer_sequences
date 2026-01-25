/// Number of partitions of n into at most 4 parts.
/// https://oeis.org/A000087

pub struct A000087;

impl crate::traits::IntegerSequence for A000087 {
    const NAME: &str = "1 + floor(n + n^2/2 + n^3/3 + n^4/5)";

    const HEAD: &[crate::Value] = &[
        1, 2, 5, 11, 21, 37, 61, 96, 145, 213, 305, 426, 583, 783, 1035, 1348, 1733, 2201, 2765,
        3439, 4239, 5180, 6282, 7562, 9042, 10742, 12686, 14898, 17405, 20234, 23416, 26982,
        30966, 35404, 40335, 45798, 51837, 58497, 65825, 73871, 82687, 92328, 102852, 114319,
        126793, 140338, 155024, 170920, 188102, 206646,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000087";

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
    crate::tester::test_sequance_formula_matchces_head::<A000087>();
}
