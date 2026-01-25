/// a(n) = 4*n^6
/// https://oeis.org/A000293

pub struct A000293;

impl crate::traits::IntegerSequence for A000293 {
    const NAME: &str = "a(n) = 4*n^6";

    const HEAD: &[crate::Value] = &[
        0, 4, 256, 2916, 16384, 62500, 186624, 470596, 1048576, 2125764, 4000000, 7086244, 11943936, 19307236, 30118144, 45562500, 67108864, 96550276, 136048896, 188183524, 256000000, 343064484, 453519616, 592143556, 764411904
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000293";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_293(n)
    }
}

const fn power_293(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 6 {
        result *= n;
        i += 1;
    }
    4 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000293>();
}
