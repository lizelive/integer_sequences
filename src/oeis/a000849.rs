/// a(n) = 10*T(n)^5
/// https://oeis.org/A000849

pub struct A000849;

impl crate::traits::IntegerSequence for A000849 {
    const NAME: &str = "a(n) = 10*T(n)^5";

    const HEAD: &[crate::Value] = &[
        0, 10, 2430, 77760, 1000000, 7593750, 40841010, 172103680, 604661760, 1845281250, 5032843750, 12523325760, 28871743680, 62403214510, 127628156250, 248832000000, 465258741760, 838411359930, 1462111698510, 2476099000000, 4084101000000, 6577485501510, 10365794764930, 16015681013760, 24300000000000, 36259082031250, 53276487267510, 77171865583680, 110313881997760, 155756537718750
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000849";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_849(n)
    }
}

const fn tri_pow_849(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    10 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000849>();
}
