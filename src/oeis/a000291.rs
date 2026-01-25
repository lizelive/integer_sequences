/// a(n) = 2*n^6
/// https://oeis.org/A000291

pub struct A000291;

impl crate::traits::IntegerSequence for A000291 {
    const NAME: &str = "a(n) = 2*n^6";

    const HEAD: &[crate::Value] = &[
        0, 2, 128, 1458, 8192, 31250, 93312, 235298, 524288, 1062882, 2000000, 3543122, 5971968, 9653618, 15059072, 22781250, 33554432, 48275138, 68024448, 94091762, 128000000, 171532242, 226759808, 296071778, 382205952
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000291";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_291(n)
    }
}

const fn power_291(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 6 {
        result *= n;
        i += 1;
    }
    2 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000291>();
}
