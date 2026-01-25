/// a(n) = 7*T(n)^2 + 1
/// https://oeis.org/A000866

pub struct A000866;

impl crate::traits::IntegerSequence for A000866 {
    const NAME: &str = "a(n) = 7*T(n)^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 8, 64, 253, 701, 1576, 3088, 5489, 9073, 14176, 21176, 30493, 42589, 57968, 77176, 100801, 129473, 163864, 204688, 252701, 308701, 373528, 448064, 533233, 630001, 739376, 862408, 1000189, 1153853, 1324576
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000866";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_866(n)
    }
}

const fn tri_pow_866(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    7 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000866>();
}
