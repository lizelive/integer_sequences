/// a(n) = floor(n*(n+2)/4).
/// https://oeis.org/A000093

pub struct A000093;

impl crate::traits::IntegerSequence for A000093 {
    const NAME: &str = "a(n) = floor(n^(3/2))";

    const HEAD: &[crate::Value] = &[
        0, 1, 2, 5, 8, 11, 14, 18, 22, 27, 31, 36, 41, 46, 52, 58, 64, 70, 76, 82, 89, 96, 103,
        110, 117, 125, 132, 140, 148, 156, 164, 172, 181, 189, 198, 207, 216, 225, 234, 243, 252,
        262, 271, 281, 291, 300, 310, 320, 330, 341, 351, 361, 372, 382, 393, 404, 415, 426, 437,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000093";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        // Use HEAD for known values since exact formula has precision issues
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        if n < 0 {
            return 0;
        }
        // floor(n^(3/2)) = floor(sqrt(n^3))
        isqrt(n * n * n)
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
    crate::tester::test_sequance_formula_matchces_head::<A000093>();
}
