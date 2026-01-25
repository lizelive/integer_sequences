/// a(n) = 9*T(n)^5 + 1
/// https://oeis.org/A000898

pub struct A000898;

impl crate::traits::IntegerSequence for A000898 {
    const NAME: &str = "a(n) = 9*T(n)^5 + 1";

    const HEAD: &[crate::Value] = &[
        1, 10, 2188, 69985, 900001, 6834376, 36756910, 154893313, 544195585, 1660753126, 4529559376, 11270993185, 25984569313, 56162893060, 114865340626, 223948800001, 418732867585, 754570223938, 1315900528660, 2228489100001, 3675690900001, 5919736951360, 9329215288438, 14414112912385, 21870000000001, 32633173828126, 47948838540760, 69454679025313, 99282493797985, 140180883946876
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000898";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_898(n)
    }
}

const fn tri_pow_898(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    9 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000898>();
}
