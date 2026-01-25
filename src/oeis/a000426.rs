/// a(n) = 6*3^n
/// https://oeis.org/A000426

pub struct A000426;

impl crate::traits::IntegerSequence for A000426 {
    const NAME: &str = "a(n) = 6*3^n";

    const HEAD: &[crate::Value] = &[
        6, 18, 54, 162, 486, 1458, 4374, 13122, 39366, 118098, 354294, 1062882, 3188646, 9565938, 28697814, 86093442, 258280326, 774840978, 2324522934, 6973568802, 20920706406, 62762119218, 188286357654, 564859072962, 1694577218886
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000426";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_426(n)
    }
}

const fn pow_426(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 3;
        i += 1;
    }
    6 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000426>();
}
