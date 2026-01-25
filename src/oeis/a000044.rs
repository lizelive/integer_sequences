/// Dying rabbits problem.
/// https://oeis.org/A000044

pub struct A000044;

impl crate::traits::IntegerSequence for A000044 {
    const NAME: &str = "Dying rabbits: a(n) = a(n-1) + a(n-2) - a(n-12)";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 232, 375, 606, 979, 1582, 2556, 4130, 6674,
        10781, 17421, 28145, 45474, 73460, 118713, 191820, 309914, 500726, 809129, 1307298,
        2112342, 3414078, 5517276, 8917192, 14411843, 23291330, 37647579, 60850728, 98340595,
        158931483, 256892284, 415227470, 671156038,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000044";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        0
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000044>();
}
