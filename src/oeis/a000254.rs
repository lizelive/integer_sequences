/// a(n) = 5*n^2
/// https://oeis.org/A000254

pub struct A000254;

impl crate::traits::IntegerSequence for A000254 {
    const NAME: &str = "a(n) = 5*n^2";

    const HEAD: &[crate::Value] = &[
        0, 5, 20, 45, 80, 125, 180, 245, 320, 405, 500, 605, 720, 845, 980, 1125, 1280, 1445, 1620, 1805, 2000, 2205, 2420, 2645, 2880
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000254";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_254(n)
    }
}

const fn power_254(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 2 {
        result *= n;
        i += 1;
    }
    5 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000254>();
}
