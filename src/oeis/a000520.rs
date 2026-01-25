/// a(n) = n^3 + 0*n + 2
/// https://oeis.org/A000520

pub struct A000520;

impl crate::traits::IntegerSequence for A000520 {
    const NAME: &str = "a(n) = n^3 + 0*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 3, 10, 29, 66, 127, 218, 345, 514, 731, 1002, 1333, 1730, 2199, 2746, 3377, 4098, 4915, 5834, 6861, 8002, 9263, 10650, 12169, 13826, 15627, 17578, 19685, 21954, 24391
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000520";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_520(n)
    }
}

const fn poly_520(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 0 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000520>();
}
