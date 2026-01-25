/// a(n) = 5*T(n)^4
/// https://oeis.org/A000834

pub struct A000834;

impl crate::traits::IntegerSequence for A000834 {
    const NAME: &str = "a(n) = 5*T(n)^4";

    const HEAD: &[crate::Value] = &[
        0, 5, 405, 6480, 50000, 253125, 972405, 3073280, 8398080, 20503125, 45753125, 94873680, 185075280, 342874805, 607753125, 1036800000, 1710510080, 2739906405, 4275180405, 6516050000, 9724050000, 14236981605, 20485760405, 29013914880, 40500000000, 55783203125, 75892432005, 102079187280, 135854534480, 179030503125
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000834";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_834(n)
    }
}

const fn tri_pow_834(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    5 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000834>();
}
