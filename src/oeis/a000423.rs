/// a(n) = 5*7^n
/// https://oeis.org/A000423

pub struct A000423;

impl crate::traits::IntegerSequence for A000423 {
    const NAME: &str = "a(n) = 5*7^n";

    const HEAD: &[crate::Value] = &[
        5, 35, 245, 1715, 12005, 84035, 588245, 4117715, 28824005, 201768035, 1412376245, 9886633715, 69206436005, 484445052035, 3391115364245, 23737807549715, 166164652848005, 1163152569936035, 8142067989552245, 56994475926865715, 398961331488060005
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000423";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_423(n)
    }
}

const fn pow_423(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 7;
        i += 1;
    }
    5 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000423>();
}
