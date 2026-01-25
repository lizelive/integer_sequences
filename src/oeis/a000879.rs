/// a(n) = 10*T(n)^3 + 1
/// https://oeis.org/A000879

pub struct A000879;

impl crate::traits::IntegerSequence for A000879 {
    const NAME: &str = "a(n) = 10*T(n)^3 + 1";

    const HEAD: &[crate::Value] = &[
        1, 11, 271, 2161, 10001, 33751, 92611, 219521, 466561, 911251, 1663751, 2874961, 4745521, 7535711, 11576251, 17280001, 25154561, 35815771, 50002111, 68590001, 92610001, 123263911, 161942771, 210245761, 270000001, 343281251, 432435511, 540101521, 669234161, 823128751
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000879";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_879(n)
    }
}

const fn tri_pow_879(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    10 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000879>();
}
