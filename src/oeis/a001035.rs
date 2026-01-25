/// a(n) = 6*n^3 + 0*n^2 + 2*n
/// https://oeis.org/A001035

pub struct A001035;

impl crate::traits::IntegerSequence for A001035 {
    const NAME: &str = "a(n) = 6*n^3 + 0*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 8, 52, 168, 392, 760, 1308, 2072, 3088, 4392, 6020, 8008, 10392, 13208, 16492, 20280, 24608, 29512, 35028, 41192, 48040, 55608, 63932, 73048, 82992, 93800, 105508, 118152, 131768, 146392
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001035";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1035(n)
    }
}

const fn cubic_1035(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n * n + 0 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001035>();
}
