/// a(n) = 3*T(n)^2 + 1
/// https://oeis.org/A000862

pub struct A000862;

impl crate::traits::IntegerSequence for A000862 {
    const NAME: &str = "a(n) = 3*T(n)^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 4, 28, 109, 301, 676, 1324, 2353, 3889, 6076, 9076, 13069, 18253, 24844, 33076, 43201, 55489, 70228, 87724, 108301, 132301, 160084, 192028, 228529, 270001, 316876, 369604, 428653, 494509, 567676
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000862";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_862(n)
    }
}

const fn tri_pow_862(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    3 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000862>();
}
