/// a(n) = n^3 + 8*n + 5
/// https://oeis.org/A000558

pub struct A000558;

impl crate::traits::IntegerSequence for A000558 {
    const NAME: &str = "a(n) = n^3 + 8*n + 5";

    const HEAD: &[crate::Value] = &[
        5, 14, 29, 56, 101, 170, 269, 404, 581, 806, 1085, 1424, 1829, 2306, 2861, 3500, 4229, 5054, 5981, 7016, 8165, 9434, 10829, 12356, 14021, 15830, 17789, 19904, 22181, 24626
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000558";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_558(n)
    }
}

const fn poly_558(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 8 * n + 5
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000558>();
}
