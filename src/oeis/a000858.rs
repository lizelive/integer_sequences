/// a(n) = 9*T(n)^1 + 1
/// https://oeis.org/A000858

pub struct A000858;

impl crate::traits::IntegerSequence for A000858 {
    const NAME: &str = "a(n) = 9*T(n)^1 + 1";

    const HEAD: &[crate::Value] = &[
        1, 10, 28, 55, 91, 136, 190, 253, 325, 406, 496, 595, 703, 820, 946, 1081, 1225, 1378, 1540, 1711, 1891, 2080, 2278, 2485, 2701, 2926, 3160, 3403, 3655, 3916
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000858";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_858(n)
    }
}

const fn tri_pow_858(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 1 {
        result *= t;
        i += 1;
    }
    9 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000858>();
}
