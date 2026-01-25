/// a(n) = 10*n^2 + 4
/// https://oeis.org/A000949

pub struct A000949;

impl crate::traits::IntegerSequence for A000949 {
    const NAME: &str = "a(n) = 10*n^2 + 4";

    const HEAD: &[crate::Value] = &[
        4, 14, 44, 94, 164, 254, 364, 494, 644, 814, 1004, 1214, 1444, 1694, 1964, 2254, 2564, 2894, 3244, 3614, 4004, 4414, 4844, 5294, 5764, 6254, 6764, 7294, 7844, 8414
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000949";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_949(n)
    }
}

const fn sq_949(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    10 * n * n + 4 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000949>();
}
