/// a(n) = 1*T(n)^3
/// https://oeis.org/A000820

pub struct A000820;

impl crate::traits::IntegerSequence for A000820 {
    const NAME: &str = "a(n) = 1*T(n)^3";

    const HEAD: &[crate::Value] = &[
        0, 1, 27, 216, 1000, 3375, 9261, 21952, 46656, 91125, 166375, 287496, 474552, 753571, 1157625, 1728000, 2515456, 3581577, 5000211, 6859000, 9261000, 12326391, 16194277, 21024576, 27000000, 34328125, 43243551, 54010152, 66923416, 82312875
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000820";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_820(n)
    }
}

const fn tri_pow_820(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    1 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000820>();
}
