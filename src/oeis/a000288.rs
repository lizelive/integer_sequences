/// a(n) = 9*n^5
/// https://oeis.org/A000288

pub struct A000288;

impl crate::traits::IntegerSequence for A000288 {
    const NAME: &str = "a(n) = 9*n^5";

    const HEAD: &[crate::Value] = &[
        0, 9, 288, 2187, 9216, 28125, 69984, 151263, 294912, 531441, 900000, 1449459, 2239488, 3341637, 4840416, 6834375, 9437184, 12778713, 17006112, 22284891, 28800000, 36756909, 46382688, 57927087, 71663616
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000288";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_288(n)
    }
}

const fn power_288(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 5 {
        result *= n;
        i += 1;
    }
    9 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000288>();
}
