/// a(n) = 2*n^2 + 4*n + 0
/// https://oeis.org/A000728

pub struct A000728;

impl crate::traits::IntegerSequence for A000728 {
    const NAME: &str = "a(n) = 2*n^2 + 4*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 6, 16, 30, 48, 70, 96, 126, 160, 198, 240, 286, 336, 390, 448, 510, 576, 646, 720, 798, 880, 966, 1056, 1150, 1248, 1350, 1456, 1566, 1680, 1798
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000728";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_728(n)
    }
}

const fn quad_728(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 4 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000728>();
}
