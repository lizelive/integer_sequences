/// a(n) = 7*T(n)^2
/// https://oeis.org/A000816

pub struct A000816;

impl crate::traits::IntegerSequence for A000816 {
    const NAME: &str = "a(n) = 7*T(n)^2";

    const HEAD: &[crate::Value] = &[
        0, 7, 63, 252, 700, 1575, 3087, 5488, 9072, 14175, 21175, 30492, 42588, 57967, 77175, 100800, 129472, 163863, 204687, 252700, 308700, 373527, 448063, 533232, 630000, 739375, 862407, 1000188, 1153852, 1324575
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000816";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_816(n)
    }
}

const fn tri_pow_816(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    7 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000816>();
}
