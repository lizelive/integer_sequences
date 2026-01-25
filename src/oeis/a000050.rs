/// a(n) = round(2^n / n).
/// https://oeis.org/A000050

pub struct A000050;

impl crate::traits::IntegerSequence for A000050 {
    const NAME: &str = "a(n) = round(2^n / n)";

    const HEAD: &[crate::Value] = &[
        1, 2, 1, 1, 1, 1, 2, 3, 4, 6, 9, 13, 19, 28, 41, 60, 89, 131, 194, 287, 426, 632, 940,
        1399, 2085, 3108, 4638, 6924, 10347, 15467, 23138, 34636, 51858, 77672, 116370, 174406,
        261438, 391937, 587636, 881115, 1321284, 1981464, 2971639, 4456785, 6684368, 10025573,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000050";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        // Use HEAD for known values since formula has edge cases
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        if n <= 0 {
            return 1;
        }
        let pow2n = 1isize << n;
        (pow2n + n / 2) / n
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000050>();
}
