/// a(n) = 2*n^4
/// https://oeis.org/A000271

pub struct A000271;

impl crate::traits::IntegerSequence for A000271 {
    const NAME: &str = "a(n) = 2*n^4";

    const HEAD: &[crate::Value] = &[
        0, 2, 32, 162, 512, 1250, 2592, 4802, 8192, 13122, 20000, 29282, 41472, 57122, 76832, 101250, 131072, 167042, 209952, 260642, 320000, 388962, 468512, 559682, 663552
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000271";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_271(n)
    }
}

const fn power_271(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 4 {
        result *= n;
        i += 1;
    }
    2 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000271>();
}
