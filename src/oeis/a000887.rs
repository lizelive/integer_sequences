/// a(n) = 8*T(n)^4 + 1
/// https://oeis.org/A000887

pub struct A000887;

impl crate::traits::IntegerSequence for A000887 {
    const NAME: &str = "a(n) = 8*T(n)^4 + 1";

    const HEAD: &[crate::Value] = &[
        1, 9, 649, 10369, 80001, 405001, 1555849, 4917249, 13436929, 32805001, 73205001, 151797889, 296120449, 548599689, 972405001, 1658880001, 2736816129, 4383850249, 6840288649, 10425680001, 15558480001, 22779170569, 32777216649, 46422263809, 64800000001, 89253125001, 121427891209, 163326699649, 217367255169, 286448805001
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000887";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_887(n)
    }
}

const fn tri_pow_887(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    8 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000887>();
}
