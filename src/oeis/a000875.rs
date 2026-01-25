/// a(n) = 6*T(n)^3 + 1
/// https://oeis.org/A000875

pub struct A000875;

impl crate::traits::IntegerSequence for A000875 {
    const NAME: &str = "a(n) = 6*T(n)^3 + 1";

    const HEAD: &[crate::Value] = &[
        1, 7, 163, 1297, 6001, 20251, 55567, 131713, 279937, 546751, 998251, 1724977, 2847313, 4521427, 6945751, 10368001, 15092737, 21489463, 30001267, 41154001, 55566001, 73958347, 97165663, 126147457, 162000001, 205968751, 259461307, 324060913, 401540497, 493877251
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000875";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_875(n)
    }
}

const fn tri_pow_875(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    6 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000875>();
}
