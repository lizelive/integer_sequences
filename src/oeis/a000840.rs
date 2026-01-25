/// a(n) = 1*T(n)^5
/// https://oeis.org/A000840

pub struct A000840;

impl crate::traits::IntegerSequence for A000840 {
    const NAME: &str = "a(n) = 1*T(n)^5";

    const HEAD: &[crate::Value] = &[
        0, 1, 243, 7776, 100000, 759375, 4084101, 17210368, 60466176, 184528125, 503284375, 1252332576, 2887174368, 6240321451, 12762815625, 24883200000, 46525874176, 83841135993, 146211169851, 247609900000, 408410100000, 657748550151, 1036579476493, 1601568101376, 2430000000000, 3625908203125, 5327648726751, 7717186558368, 11031388199776, 15575653771875
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000840";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_840(n)
    }
}

const fn tri_pow_840(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    1 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000840>();
}
