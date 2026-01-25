/// a(n) = (n+1)*n^2*(n-1)/4
/// https://oeis.org/A000120

pub struct A000120;

impl crate::traits::IntegerSequence for A000120 {
    const NAME: &str = "a(n) = (n+1)*n^2*(n-1)/4";

    const HEAD: &[crate::Value] = &[
        0, 0, 3, 18, 60, 150, 315, 588, 1008, 1620, 2475, 3630, 5148, 7098, 9555, 12600, 16320
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000120";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        formula_120(n)
    }
}

const fn formula_120(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    (n + 1) * n * n * (n - 1) / 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000120>();
}
