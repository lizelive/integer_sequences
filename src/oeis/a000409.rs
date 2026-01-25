/// a(n) = 2*11^n
/// https://oeis.org/A000409

pub struct A000409;

impl crate::traits::IntegerSequence for A000409 {
    const NAME: &str = "a(n) = 2*11^n";

    const HEAD: &[crate::Value] = &[
        2, 22, 242, 2662, 29282, 322102, 3543122, 38974342, 428717762, 4715895382, 51874849202, 570623341222, 6276856753442, 69045424287862, 759499667166482, 8354496338831302, 91899459727144322
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000409";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_409(n)
    }
}

const fn pow_409(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 11;
        i += 1;
    }
    2 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000409>();
}
