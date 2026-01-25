/// a(n) = n^3 + 2*n^2 + 0*n + 1
/// https://oeis.org/A000227

pub struct A000227;

impl crate::traits::IntegerSequence for A000227 {
    const NAME: &str = "a(n) = n^3 + 2*n^2 + 0*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 4, 17, 46, 97, 176, 289, 442, 641, 892, 1201, 1574, 2017, 2536, 3137, 3826, 4609, 5492, 6481, 7582, 8801, 10144, 11617, 13226, 14977
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000227";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_227(n)
    }
}

const fn poly_227(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n * n + 0 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000227>();
}
