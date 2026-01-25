/// a(n) = 7*n^2 + 7
/// https://oeis.org/A000976

pub struct A000976;

impl crate::traits::IntegerSequence for A000976 {
    const NAME: &str = "a(n) = 7*n^2 + 7";

    const HEAD: &[crate::Value] = &[
        7, 14, 35, 70, 119, 182, 259, 350, 455, 574, 707, 854, 1015, 1190, 1379, 1582, 1799, 2030, 2275, 2534, 2807, 3094, 3395, 3710, 4039, 4382, 4739, 5110, 5495, 5894
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000976";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_976(n)
    }
}

const fn sq_976(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    7 * n * n + 7 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000976>();
}
