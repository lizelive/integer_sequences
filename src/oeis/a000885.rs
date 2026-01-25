/// a(n) = 6*T(n)^4 + 1
/// https://oeis.org/A000885

pub struct A000885;

impl crate::traits::IntegerSequence for A000885 {
    const NAME: &str = "a(n) = 6*T(n)^4 + 1";

    const HEAD: &[crate::Value] = &[
        1, 7, 487, 7777, 60001, 303751, 1166887, 3687937, 10077697, 24603751, 54903751, 113848417, 222090337, 411449767, 729303751, 1244160001, 2052612097, 3287887687, 5130216487, 7819260001, 11668860001, 17084377927, 24582912487, 34816697857, 48600000001, 66939843751, 91070918407, 122495024737, 163025441377, 214836603751
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000885";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_885(n)
    }
}

const fn tri_pow_885(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    6 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000885>();
}
