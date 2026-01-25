/// a(n) = 2*n^3 + 2*n^2 + 1*n
/// https://oeis.org/A001013

pub struct A001013;

impl crate::traits::IntegerSequence for A001013 {
    const NAME: &str = "a(n) = 2*n^3 + 2*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 5, 26, 75, 164, 305, 510, 791, 1160, 1629, 2210, 2915, 3756, 4745, 5894, 7215, 8720, 10421, 12330, 14459, 16820, 19425, 22286, 25415, 28824, 32525, 36530, 40851, 45500, 50489
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001013";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1013(n)
    }
}

const fn cubic_1013(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n * n + 2 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001013>();
}
