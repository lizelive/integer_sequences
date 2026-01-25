/// a(n) = 8*T(n)^4
/// https://oeis.org/A000837

pub struct A000837;

impl crate::traits::IntegerSequence for A000837 {
    const NAME: &str = "a(n) = 8*T(n)^4";

    const HEAD: &[crate::Value] = &[
        0, 8, 648, 10368, 80000, 405000, 1555848, 4917248, 13436928, 32805000, 73205000, 151797888, 296120448, 548599688, 972405000, 1658880000, 2736816128, 4383850248, 6840288648, 10425680000, 15558480000, 22779170568, 32777216648, 46422263808, 64800000000, 89253125000, 121427891208, 163326699648, 217367255168, 286448805000
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000837";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_837(n)
    }
}

const fn tri_pow_837(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    8 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000837>();
}
