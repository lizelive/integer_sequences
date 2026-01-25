/// a(n) = 2*T(n)^5
/// https://oeis.org/A000841

pub struct A000841;

impl crate::traits::IntegerSequence for A000841 {
    const NAME: &str = "a(n) = 2*T(n)^5";

    const HEAD: &[crate::Value] = &[
        0, 2, 486, 15552, 200000, 1518750, 8168202, 34420736, 120932352, 369056250, 1006568750, 2504665152, 5774348736, 12480642902, 25525631250, 49766400000, 93051748352, 167682271986, 292422339702, 495219800000, 816820200000, 1315497100302, 2073158952986, 3203136202752, 4860000000000, 7251816406250, 10655297453502, 15434373116736, 22062776399552, 31151307543750
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000841";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_841(n)
    }
}

const fn tri_pow_841(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    2 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000841>();
}
