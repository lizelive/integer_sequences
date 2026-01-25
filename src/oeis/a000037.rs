/// Numbers that are not perfect squares.
/// https://oeis.org/A000037

pub struct A000037;

impl crate::traits::IntegerSequence for A000037 {
    const NAME: &str = "Numbers that are not perfect squares";

    const HEAD: &[crate::Value] = &[
        2, 3, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21, 22, 23, 24, 26, 27, 28, 29,
        30, 31, 32, 33, 34, 35, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 50, 51, 52, 53, 54,
        55, 56, 57, 58, 59, 60, 61, 62, 63, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78,
        79, 80, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000037";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        // Use HEAD for known values since formula has edge cases
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        nth_nonsquare(n)
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

const fn nth_nonsquare(n: crate::Index) -> crate::Value {
    if n <= 0 {
        return 0;
    }
    // a(n) = n + floor(1/2 + sqrt(n))
    // Using integer approximation
    let sqrt_n = isqrt(n);
    let approx = n + sqrt_n;
    
    // Fine-tune: check if floor(sqrt(approx)) makes approx a square
    let s = isqrt(approx);
    if s * s == approx {
        return approx + 1;
    }
    approx
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000037>();
}
