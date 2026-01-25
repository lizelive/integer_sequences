/// a(n) = 5*T(n)^4 + 1
/// https://oeis.org/A000884

pub struct A000884;

impl crate::traits::IntegerSequence for A000884 {
    const NAME: &str = "a(n) = 5*T(n)^4 + 1";

    const HEAD: &[crate::Value] = &[
        1, 6, 406, 6481, 50001, 253126, 972406, 3073281, 8398081, 20503126, 45753126, 94873681, 185075281, 342874806, 607753126, 1036800001, 1710510081, 2739906406, 4275180406, 6516050001, 9724050001, 14236981606, 20485760406, 29013914881, 40500000001, 55783203126, 75892432006, 102079187281, 135854534481, 179030503126
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000884";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_884(n)
    }
}

const fn tri_pow_884(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    5 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000884>();
}
