/// a(n) = 2*T(n)^2 + 1
/// https://oeis.org/A000861

pub struct A000861;

impl crate::traits::IntegerSequence for A000861 {
    const NAME: &str = "a(n) = 2*T(n)^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 3, 19, 73, 201, 451, 883, 1569, 2593, 4051, 6051, 8713, 12169, 16563, 22051, 28801, 36993, 46819, 58483, 72201, 88201, 106723, 128019, 152353, 180001, 211251, 246403, 285769, 329673, 378451
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000861";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_861(n)
    }
}

const fn tri_pow_861(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    2 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000861>();
}
