/// a(n) = 2*n^2 + n = n*(2n+1)
/// https://oeis.org/A000149

pub struct A000149;

impl crate::traits::IntegerSequence for A000149 {
    const NAME: &str = "a(n) = 2*n^2 + n";

    const HEAD: &[crate::Value] = &[
        0, 3, 10, 21, 36, 55, 78, 105, 136, 171, 210, 253, 300, 351, 406, 465, 528, 595, 666, 741, 820, 903, 990, 1081, 1176
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000149";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        twice_n_sq_plus_n(n)
    }
}

const fn twice_n_sq_plus_n(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000149>();
}
