/// A 1-Fibonacci sequence: a(n) = a(n-1) + a(n-2).
/// https://oeis.org/A000055

pub struct A000055;

impl crate::traits::IntegerSequence for A000055 {
    const NAME: &str = "Number of trees with n unlabeled nodes";

    const HEAD: &[crate::Value] = &[
        1, 1, 1, 1, 2, 3, 6, 11, 23, 47, 106, 235, 551, 1301, 3159, 7741, 19320, 48629, 123867,
        317955, 823065, 2144505, 5623756, 14828074, 39299897, 104636890, 279793450, 751065460,
        2023443032, 5469566585, 14830871802,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000055";

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
    crate::tester::test_sequance_formula_matchces_head::<A000055>();
}
