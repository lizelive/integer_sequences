/// a(n) = 2*n*(n+1)
/// https://oeis.org/A000150

pub struct A000150;

impl crate::traits::IntegerSequence for A000150 {
    const NAME: &str = "a(n) = 2*n*(n+1)";

    const HEAD: &[crate::Value] = &[
        0, 4, 12, 24, 40, 60, 84, 112, 144, 180, 220, 264, 312, 364, 420, 480, 544, 612, 684, 760, 840, 924, 1012, 1104, 1200
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000150";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        twice_n_times_n_plus_1(n)
    }
}

const fn twice_n_times_n_plus_1(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * (n + 1)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000150>();
}
