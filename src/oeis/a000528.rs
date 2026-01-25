/// a(n) = n^3 + 8*n + 2
/// https://oeis.org/A000528

pub struct A000528;

impl crate::traits::IntegerSequence for A000528 {
    const NAME: &str = "a(n) = n^3 + 8*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 11, 26, 53, 98, 167, 266, 401, 578, 803, 1082, 1421, 1826, 2303, 2858, 3497, 4226, 5051, 5978, 7013, 8162, 9431, 10826, 12353, 14018, 15827, 17786, 19901, 22178, 24623
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000528";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_528(n)
    }
}

const fn poly_528(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 8 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000528>();
}
