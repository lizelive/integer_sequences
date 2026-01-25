/// a(n) = 3*n^3 + 3*n^2 + 2*n
/// https://oeis.org/A001050

pub struct A001050;

impl crate::traits::IntegerSequence for A001050 {
    const NAME: &str = "a(n) = 3*n^3 + 3*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 8, 40, 114, 248, 460, 768, 1190, 1744, 2448, 3320, 4378, 5640, 7124, 8848, 10830, 13088, 15640, 18504, 21698, 25240, 29148, 33440, 38134, 43248, 48800, 54808, 61290, 68264, 75748
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001050";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1050(n)
    }
}

const fn cubic_1050(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n * n + 3 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001050>();
}
