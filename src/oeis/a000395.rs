/// a(n) = 5*T(n) + 5
/// https://oeis.org/A000395

pub struct A000395;

impl crate::traits::IntegerSequence for A000395 {
    const NAME: &str = "a(n) = 5*T(n) + 5";

    const HEAD: &[crate::Value] = &[
        5, 10, 20, 35, 55, 80, 110, 145, 185, 230, 280, 335, 395, 460, 530, 605, 685, 770, 860, 955, 1055, 1160, 1270, 1385, 1505
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000395";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_395(n)
    }
}

const fn tri_395(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * (n + 1) / 2 + 5
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000395>();
}
