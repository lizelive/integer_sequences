/// a(n) = 1*n^4
/// https://oeis.org/A000270

pub struct A000270;

impl crate::traits::IntegerSequence for A000270 {
    const NAME: &str = "a(n) = 1*n^4";

    const HEAD: &[crate::Value] = &[
        0, 1, 16, 81, 256, 625, 1296, 2401, 4096, 6561, 10000, 14641, 20736, 28561, 38416, 50625, 65536, 83521, 104976, 130321, 160000, 194481, 234256, 279841, 331776
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000270";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_270(n)
    }
}

const fn power_270(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 4 {
        result *= n;
        i += 1;
    }
    1 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000270>();
}
