/// a(n) = 5*T(n)^2
/// https://oeis.org/A000814

pub struct A000814;

impl crate::traits::IntegerSequence for A000814 {
    const NAME: &str = "a(n) = 5*T(n)^2";

    const HEAD: &[crate::Value] = &[
        0, 5, 45, 180, 500, 1125, 2205, 3920, 6480, 10125, 15125, 21780, 30420, 41405, 55125, 72000, 92480, 117045, 146205, 180500, 220500, 266805, 320045, 380880, 450000, 528125, 616005, 714420, 824180, 946125
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000814";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_814(n)
    }
}

const fn tri_pow_814(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    5 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000814>();
}
