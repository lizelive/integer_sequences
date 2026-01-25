/// a(n) = 6*11^n
/// https://oeis.org/A000429

pub struct A000429;

impl crate::traits::IntegerSequence for A000429 {
    const NAME: &str = "a(n) = 6*11^n";

    const HEAD: &[crate::Value] = &[
        6, 66, 726, 7986, 87846, 966306, 10629366, 116923026, 1286153286, 14147686146, 155624547606, 1711870023666, 18830570260326, 207136272863586, 2278499001499446, 25063489016493906, 275698379181432966
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000429";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_429(n)
    }
}

const fn pow_429(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 11;
        i += 1;
    }
    6 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000429>();
}
