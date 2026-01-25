/// a(n) = n^3 + 2*n^2 + 2*n + 1
/// https://oeis.org/A000237

pub struct A000237;

impl crate::traits::IntegerSequence for A000237 {
    const NAME: &str = "a(n) = n^3 + 2*n^2 + 2*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 6, 21, 52, 105, 186, 301, 456, 657, 910, 1221, 1596, 2041, 2562, 3165, 3856, 4641, 5526, 6517, 7620, 8841, 10186, 11661, 13272, 15025
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000237";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_237(n)
    }
}

const fn poly_237(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n * n + 2 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000237>();
}
