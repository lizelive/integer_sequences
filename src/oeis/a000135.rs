/// a(n) = n*(n+1)*(n+2) = (n+2)!/((n-1)!) for n>0
/// https://oeis.org/A000135

pub struct A000135;

impl crate::traits::IntegerSequence for A000135 {
    const NAME: &str = "a(n) = n*(n+1)*(n+2)";

    const HEAD: &[crate::Value] = &[
        0, 6, 24, 60, 120, 210, 336, 504, 720, 990, 1320, 1716, 2184, 2730, 3360, 4080, 4896, 5814, 6840, 7980, 9240, 10626, 12144, 13800, 15600, 17550, 19656, 21924, 24360, 26970
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000135";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        triple_rising(n)
    }
}

const fn triple_rising(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * (n + 1) * (n + 2)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000135>();
}
