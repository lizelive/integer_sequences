/// a(n) = 3*n^3 + 2*n^2 + 2*n
/// https://oeis.org/A001044

pub struct A001044;

impl crate::traits::IntegerSequence for A001044 {
    const NAME: &str = "a(n) = 3*n^3 + 2*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 7, 36, 105, 232, 435, 732, 1141, 1680, 2367, 3220, 4257, 5496, 6955, 8652, 10605, 12832, 15351, 18180, 21337, 24840, 28707, 32956, 37605, 42672, 48175, 54132, 60561, 67480, 74907
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001044";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1044(n)
    }
}

const fn cubic_1044(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n * n + 2 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001044>();
}
