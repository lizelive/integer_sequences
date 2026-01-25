/// a(n) = 1*n^2 + 2*n + 2
/// https://oeis.org/A000711

pub struct A000711;

impl crate::traits::IntegerSequence for A000711 {
    const NAME: &str = "a(n) = 1*n^2 + 2*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 5, 10, 17, 26, 37, 50, 65, 82, 101, 122, 145, 170, 197, 226, 257, 290, 325, 362, 401, 442, 485, 530, 577, 626, 677, 730, 785, 842, 901
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000711";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_711(n)
    }
}

const fn quad_711(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 2 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000711>();
}
