/// a(n) = 10*n^2 + 0
/// https://oeis.org/A000909

pub struct A000909;

impl crate::traits::IntegerSequence for A000909 {
    const NAME: &str = "a(n) = 10*n^2 + 0";

    const HEAD: &[crate::Value] = &[
        0, 10, 40, 90, 160, 250, 360, 490, 640, 810, 1000, 1210, 1440, 1690, 1960, 2250, 2560, 2890, 3240, 3610, 4000, 4410, 4840, 5290, 5760, 6250, 6760, 7290, 7840, 8410
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000909";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_909(n)
    }
}

const fn sq_909(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    10 * n * n + 0 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000909>();
}
