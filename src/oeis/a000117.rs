/// a(n) = n^2*(n+2)
/// https://oeis.org/A000117

pub struct A000117;

impl crate::traits::IntegerSequence for A000117 {
    const NAME: &str = "a(n) = n^2*(n+2)";

    const HEAD: &[crate::Value] = &[
        0, 3, 16, 45, 96, 175, 288, 441, 640, 891, 1200, 1573, 2016, 2535, 3136, 3825, 4608, 5491
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000117";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        n_sq_times_n_plus_2(n)
    }
}

const fn n_sq_times_n_plus_2(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * (n + 2)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000117>();
}
