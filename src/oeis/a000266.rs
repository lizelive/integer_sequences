/// a(n) = 7*n^3
/// https://oeis.org/A000266

pub struct A000266;

impl crate::traits::IntegerSequence for A000266 {
    const NAME: &str = "a(n) = 7*n^3";

    const HEAD: &[crate::Value] = &[
        0, 7, 56, 189, 448, 875, 1512, 2401, 3584, 5103, 7000, 9317, 12096, 15379, 19208, 23625, 28672, 34391, 40824, 48013, 56000, 64827, 74536, 85169, 96768
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000266";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_266(n)
    }
}

const fn power_266(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 3 {
        result *= n;
        i += 1;
    }
    7 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000266>();
}
