/// a(n) = 3*n^2 + 7
/// https://oeis.org/A000972

pub struct A000972;

impl crate::traits::IntegerSequence for A000972 {
    const NAME: &str = "a(n) = 3*n^2 + 7";

    const HEAD: &[crate::Value] = &[
        7, 10, 19, 34, 55, 82, 115, 154, 199, 250, 307, 370, 439, 514, 595, 682, 775, 874, 979, 1090, 1207, 1330, 1459, 1594, 1735, 1882, 2035, 2194, 2359, 2530
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000972";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_972(n)
    }
}

const fn sq_972(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 7 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000972>();
}
