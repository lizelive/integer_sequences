/// a(n) = 5*n^3 + 1*n^2 + 2*n
/// https://oeis.org/A001040

pub struct A001040;

impl crate::traits::IntegerSequence for A001040 {
    const NAME: &str = "a(n) = 5*n^3 + 1*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 8, 48, 150, 344, 660, 1128, 1778, 2640, 3744, 5120, 6798, 8808, 11180, 13944, 17130, 20768, 24888, 29520, 34694, 40440, 46788, 53768, 61410, 69744, 78800, 88608, 99198, 110600, 122844
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001040";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1040(n)
    }
}

const fn cubic_1040(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n * n + 1 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001040>();
}
