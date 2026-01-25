/// a(n) = 10*T(n)^5 + 1
/// https://oeis.org/A000899

pub struct A000899;

impl crate::traits::IntegerSequence for A000899 {
    const NAME: &str = "a(n) = 10*T(n)^5 + 1";

    const HEAD: &[crate::Value] = &[
        1, 11, 2431, 77761, 1000001, 7593751, 40841011, 172103681, 604661761, 1845281251, 5032843751, 12523325761, 28871743681, 62403214511, 127628156251, 248832000001, 465258741761, 838411359931, 1462111698511, 2476099000001, 4084101000001, 6577485501511, 10365794764931, 16015681013761, 24300000000001, 36259082031251, 53276487267511, 77171865583681, 110313881997761, 155756537718751
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000899";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_899(n)
    }
}

const fn tri_pow_899(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    10 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000899>();
}
