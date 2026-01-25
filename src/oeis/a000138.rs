/// a(n) = n*(n+1)*(n+2)*(n+3)/6
/// https://oeis.org/A000138

pub struct A000138;

impl crate::traits::IntegerSequence for A000138 {
    const NAME: &str = "4D simplex numbers times 4";

    const HEAD: &[crate::Value] = &[
        0, 4, 20, 60, 140, 280, 504, 840, 1320, 1980, 2860, 4004, 5460, 7280, 9520, 12240, 15504, 19380, 23940, 29260, 35420, 42504, 50600, 59800, 70200, 81900, 95004, 109620, 125860, 143840
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000138";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_rising_div6(n)
    }
}

const fn quad_rising_div6(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * (n + 1) * (n + 2) * (n + 3) / 6
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000138>();
}
