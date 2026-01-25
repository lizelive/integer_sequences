/// a(n) = 8*n^6
/// https://oeis.org/A000297

pub struct A000297;

impl crate::traits::IntegerSequence for A000297 {
    const NAME: &str = "a(n) = 8*n^6";

    const HEAD: &[crate::Value] = &[
        0, 8, 512, 5832, 32768, 125000, 373248, 941192, 2097152, 4251528, 8000000, 14172488, 23887872, 38614472, 60236288, 91125000, 134217728, 193100552, 272097792, 376367048, 512000000, 686128968, 907039232, 1184287112, 1528823808
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000297";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_297(n)
    }
}

const fn power_297(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 6 {
        result *= n;
        i += 1;
    }
    8 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000297>();
}
