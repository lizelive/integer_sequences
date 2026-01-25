/// a(n) = 5*T(n)^1
/// https://oeis.org/A000804

pub struct A000804;

impl crate::traits::IntegerSequence for A000804 {
    const NAME: &str = "a(n) = 5*T(n)^1";

    const HEAD: &[crate::Value] = &[
        0, 5, 15, 30, 50, 75, 105, 140, 180, 225, 275, 330, 390, 455, 525, 600, 680, 765, 855, 950, 1050, 1155, 1265, 1380, 1500, 1625, 1755, 1890, 2030, 2175
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000804";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_804(n)
    }
}

const fn tri_pow_804(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 1 {
        result *= t;
        i += 1;
    }
    5 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000804>();
}
