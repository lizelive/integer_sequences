/// a(n) = n^3 + 7*n + 0
/// https://oeis.org/A000507

pub struct A000507;

impl crate::traits::IntegerSequence for A000507 {
    const NAME: &str = "a(n) = n^3 + 7*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 8, 22, 48, 92, 160, 258, 392, 568, 792, 1070, 1408, 1812, 2288, 2842, 3480, 4208, 5032, 5958, 6992, 8140, 9408, 10802, 12328, 13992, 15800, 17758, 19872, 22148, 24592
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000507";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_507(n)
    }
}

const fn poly_507(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 7 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000507>();
}
