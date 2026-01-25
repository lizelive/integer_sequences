/// a(n) = 1*n^3 + 2*n^2 + 2*n
/// https://oeis.org/A001042

pub struct A001042;

impl crate::traits::IntegerSequence for A001042 {
    const NAME: &str = "a(n) = 1*n^3 + 2*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 5, 20, 51, 104, 185, 300, 455, 656, 909, 1220, 1595, 2040, 2561, 3164, 3855, 4640, 5525, 6516, 7619, 8840, 10185, 11660, 13271, 15024, 16925, 18980, 21195, 23576, 26129
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001042";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1042(n)
    }
}

const fn cubic_1042(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n * n + 2 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001042>();
}
