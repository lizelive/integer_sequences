/// a(n) = n^3 + 0*n + 3
/// https://oeis.org/A000530

pub struct A000530;

impl crate::traits::IntegerSequence for A000530 {
    const NAME: &str = "a(n) = n^3 + 0*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 4, 11, 30, 67, 128, 219, 346, 515, 732, 1003, 1334, 1731, 2200, 2747, 3378, 4099, 4916, 5835, 6862, 8003, 9264, 10651, 12170, 13827, 15628, 17579, 19686, 21955, 24392
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000530";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_530(n)
    }
}

const fn poly_530(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 0 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000530>();
}
