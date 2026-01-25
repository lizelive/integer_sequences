/// a(n) = floor(n^3/3)
/// https://oeis.org/A000132

pub struct A000132;

impl crate::traits::IntegerSequence for A000132 {
    const NAME: &str = "a(n) = floor(n^3/3)";

    const HEAD: &[crate::Value] = &[
        0, 0, 2, 9, 21, 41, 72, 114, 170, 243, 333, 443, 576, 732, 914, 1125, 1365, 1637, 1944, 2286, 2666, 3087, 3549, 4055, 4608, 5208, 5858, 6561, 7317, 8129
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000132";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        n_cubed_div_3(n)
    }
}

const fn n_cubed_div_3(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n / 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000132>();
}
