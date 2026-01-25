/// a(n) = 6*T(n)^2 + 1
/// https://oeis.org/A000865

pub struct A000865;

impl crate::traits::IntegerSequence for A000865 {
    const NAME: &str = "a(n) = 6*T(n)^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 7, 55, 217, 601, 1351, 2647, 4705, 7777, 12151, 18151, 26137, 36505, 49687, 66151, 86401, 110977, 140455, 175447, 216601, 264601, 320167, 384055, 457057, 540001, 633751, 739207, 857305, 989017, 1135351
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000865";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_865(n)
    }
}

const fn tri_pow_865(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    6 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000865>();
}
