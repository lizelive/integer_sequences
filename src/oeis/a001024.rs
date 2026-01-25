/// a(n) = 1*n^3 + 4*n^2 + 1*n
/// https://oeis.org/A001024

pub struct A001024;

impl crate::traits::IntegerSequence for A001024 {
    const NAME: &str = "a(n) = 1*n^3 + 4*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 6, 26, 66, 132, 230, 366, 546, 776, 1062, 1410, 1826, 2316, 2886, 3542, 4290, 5136, 6086, 7146, 8322, 9620, 11046, 12606, 14306, 16152, 18150, 20306, 22626, 25116, 27782
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001024";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1024(n)
    }
}

const fn cubic_1024(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n * n + 4 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001024>();
}
