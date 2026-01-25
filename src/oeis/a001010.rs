/// a(n) = 5*n^3 + 1*n^2 + 1*n
/// https://oeis.org/A001010

pub struct A001010;

impl crate::traits::IntegerSequence for A001010 {
    const NAME: &str = "a(n) = 5*n^3 + 1*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 7, 46, 147, 340, 655, 1122, 1771, 2632, 3735, 5110, 6787, 8796, 11167, 13930, 17115, 20752, 24871, 29502, 34675, 40420, 46767, 53746, 61387, 69720, 78775, 88582, 99171, 110572, 122815
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001010";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1010(n)
    }
}

const fn cubic_1010(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n * n + 1 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001010>();
}
