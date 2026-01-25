/// a(n) = 5*n^4
/// https://oeis.org/A000274

pub struct A000274;

impl crate::traits::IntegerSequence for A000274 {
    const NAME: &str = "a(n) = 5*n^4";

    const HEAD: &[crate::Value] = &[
        0, 5, 80, 405, 1280, 3125, 6480, 12005, 20480, 32805, 50000, 73205, 103680, 142805, 192080, 253125, 327680, 417605, 524880, 651605, 800000, 972405, 1171280, 1399205, 1658880
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000274";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_274(n)
    }
}

const fn power_274(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 4 {
        result *= n;
        i += 1;
    }
    5 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000274>();
}
