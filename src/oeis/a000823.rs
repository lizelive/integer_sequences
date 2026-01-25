/// a(n) = 4*T(n)^3
/// https://oeis.org/A000823

pub struct A000823;

impl crate::traits::IntegerSequence for A000823 {
    const NAME: &str = "a(n) = 4*T(n)^3";

    const HEAD: &[crate::Value] = &[
        0, 4, 108, 864, 4000, 13500, 37044, 87808, 186624, 364500, 665500, 1149984, 1898208, 3014284, 4630500, 6912000, 10061824, 14326308, 20000844, 27436000, 37044000, 49305564, 64777108, 84098304, 108000000, 137312500, 172974204, 216040608, 267693664, 329251500
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000823";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_823(n)
    }
}

const fn tri_pow_823(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    4 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000823>();
}
