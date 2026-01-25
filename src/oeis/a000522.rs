/// a(n) = n^3 + 2*n + 2
/// https://oeis.org/A000522

pub struct A000522;

impl crate::traits::IntegerSequence for A000522 {
    const NAME: &str = "a(n) = n^3 + 2*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 5, 14, 35, 74, 137, 230, 359, 530, 749, 1022, 1355, 1754, 2225, 2774, 3407, 4130, 4949, 5870, 6899, 8042, 9305, 10694, 12215, 13874, 15677, 17630, 19739, 22010, 24449
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000522";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_522(n)
    }
}

const fn poly_522(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000522>();
}
