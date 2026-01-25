/// a(n) = 1*T(n)^5 + 1
/// https://oeis.org/A000890

pub struct A000890;

impl crate::traits::IntegerSequence for A000890 {
    const NAME: &str = "a(n) = 1*T(n)^5 + 1";

    const HEAD: &[crate::Value] = &[
        1, 2, 244, 7777, 100001, 759376, 4084102, 17210369, 60466177, 184528126, 503284376, 1252332577, 2887174369, 6240321452, 12762815626, 24883200001, 46525874177, 83841135994, 146211169852, 247609900001, 408410100001, 657748550152, 1036579476494, 1601568101377, 2430000000001, 3625908203126, 5327648726752, 7717186558369, 11031388199777, 15575653771876
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000890";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_890(n)
    }
}

const fn tri_pow_890(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    1 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000890>();
}
