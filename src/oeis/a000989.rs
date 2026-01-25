/// a(n) = 10*n^2 + 8
/// https://oeis.org/A000989

pub struct A000989;

impl crate::traits::IntegerSequence for A000989 {
    const NAME: &str = "a(n) = 10*n^2 + 8";

    const HEAD: &[crate::Value] = &[
        8, 18, 48, 98, 168, 258, 368, 498, 648, 818, 1008, 1218, 1448, 1698, 1968, 2258, 2568, 2898, 3248, 3618, 4008, 4418, 4848, 5298, 5768, 6258, 6768, 7298, 7848, 8418
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000989";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_989(n)
    }
}

const fn sq_989(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    10 * n * n + 8 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000989>();
}
