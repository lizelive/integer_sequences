/// a(n) = 1*n^3 + 3*n^2 + 1*n
/// https://oeis.org/A001018

pub struct A001018;

impl crate::traits::IntegerSequence for A001018 {
    const NAME: &str = "a(n) = 1*n^3 + 3*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 5, 22, 57, 116, 205, 330, 497, 712, 981, 1310, 1705, 2172, 2717, 3346, 4065, 4880, 5797, 6822, 7961, 9220, 10605, 12122, 13777, 15576, 17525, 19630, 21897, 24332, 26941
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001018";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1018(n)
    }
}

const fn cubic_1018(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n * n + 3 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001018>();
}
