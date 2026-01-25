/// a(n) = 9*T(n)^2 + 1
/// https://oeis.org/A000868

pub struct A000868;

impl crate::traits::IntegerSequence for A000868 {
    const NAME: &str = "a(n) = 9*T(n)^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 10, 82, 325, 901, 2026, 3970, 7057, 11665, 18226, 27226, 39205, 54757, 74530, 99226, 129601, 166465, 210682, 263170, 324901, 396901, 480250, 576082, 685585, 810001, 950626, 1108810, 1285957, 1483525, 1703026
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000868";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_868(n)
    }
}

const fn tri_pow_868(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    9 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000868>();
}
