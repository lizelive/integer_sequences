/// a(n) = 2*T(n)^5 + 1
/// https://oeis.org/A000891

pub struct A000891;

impl crate::traits::IntegerSequence for A000891 {
    const NAME: &str = "a(n) = 2*T(n)^5 + 1";

    const HEAD: &[crate::Value] = &[
        1, 3, 487, 15553, 200001, 1518751, 8168203, 34420737, 120932353, 369056251, 1006568751, 2504665153, 5774348737, 12480642903, 25525631251, 49766400001, 93051748353, 167682271987, 292422339703, 495219800001, 816820200001, 1315497100303, 2073158952987, 3203136202753, 4860000000001, 7251816406251, 10655297453503, 15434373116737, 22062776399553, 31151307543751
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000891";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_891(n)
    }
}

const fn tri_pow_891(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    2 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000891>();
}
