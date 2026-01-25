/// a(n) = 4*n^2 + 9
/// https://oeis.org/A000993

pub struct A000993;

impl crate::traits::IntegerSequence for A000993 {
    const NAME: &str = "a(n) = 4*n^2 + 9";

    const HEAD: &[crate::Value] = &[
        9, 13, 25, 45, 73, 109, 153, 205, 265, 333, 409, 493, 585, 685, 793, 909, 1033, 1165, 1305, 1453, 1609, 1773, 1945, 2125, 2313, 2509, 2713, 2925, 3145, 3373
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000993";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_993(n)
    }
}

const fn sq_993(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 9 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000993>();
}
