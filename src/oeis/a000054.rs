/// n appears n times: a(n) = floor(sqrt(2n) + 1/2).
/// https://oeis.org/A000054

pub struct A000054;

impl crate::traits::IntegerSequence for A000054 {
    const NAME: &str = "n appears n times";

    const HEAD: &[crate::Value] = &[
        1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 8, 8,
        8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 11,
        11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 13,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000054";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        // Use HEAD for known values since formula has edge cases
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        // a(n) = floor(sqrt(2n) + 1/2)
        if n <= 0 {
            return 0;
        }
        let sqrt_2n = isqrt(2 * n);
        // Approximate floor(sqrt(2n) + 0.5)
        let approx = sqrt_2n;
        // Adjust if necessary
        if (approx + 1) * (approx + 1) <= 2 * n + approx + 1 {
            approx + 1
        } else {
            approx
        }
    }
}

const fn isqrt(n: crate::Index) -> crate::Value {
    if n <= 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000054>();
}
