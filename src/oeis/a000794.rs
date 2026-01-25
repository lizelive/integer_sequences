/// a(n) = 4*n^2 + 5*n + 3
/// https://oeis.org/A000794

pub struct A000794;

impl crate::traits::IntegerSequence for A000794 {
    const NAME: &str = "a(n) = 4*n^2 + 5*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 12, 29, 54, 87, 128, 177, 234, 299, 372, 453, 542, 639, 744, 857, 978, 1107, 1244, 1389, 1542, 1703, 1872, 2049, 2234, 2427, 2628, 2837, 3054, 3279, 3512
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000794";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_794(n)
    }
}

const fn quad_794(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 5 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000794>();
}
