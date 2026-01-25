/// a(n) = 8*T(n)^2 + 1
/// https://oeis.org/A000867

pub struct A000867;

impl crate::traits::IntegerSequence for A000867 {
    const NAME: &str = "a(n) = 8*T(n)^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 9, 73, 289, 801, 1801, 3529, 6273, 10369, 16201, 24201, 34849, 48673, 66249, 88201, 115201, 147969, 187273, 233929, 288801, 352801, 426889, 512073, 609409, 720001, 845001, 985609, 1143073, 1318689, 1513801
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000867";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_867(n)
    }
}

const fn tri_pow_867(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    8 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000867>();
}
