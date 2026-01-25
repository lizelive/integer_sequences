/// Number of partitions of n with no prime parts.
/// https://oeis.org/A000067

pub struct A000067;

impl crate::traits::IntegerSequence for A000067 {
    const NAME: &str = "Convolution of natural numbers n >= 1 with Fibonacci numbers F(k) for k >= 1";

    const HEAD: &[crate::Value] = &[
        1, 3, 7, 15, 30, 57, 105, 188, 330, 570, 972, 1639, 2740, 4545, 7494, 12297, 20105, 32778,
        53328, 86627, 140560, 227883, 369238, 598022, 968274, 1567430, 2536966, 4105795, 6644306,
        10751801, 17397971, 28151810, 45551998, 73706214, 119260818, 192969843, 312233685,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000067";

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
    crate::tester::test_sequance_formula_matchces_head::<A000067>();
}
