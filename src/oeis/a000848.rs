/// a(n) = 9*T(n)^5
/// https://oeis.org/A000848

pub struct A000848;

impl crate::traits::IntegerSequence for A000848 {
    const NAME: &str = "a(n) = 9*T(n)^5";

    const HEAD: &[crate::Value] = &[
        0, 9, 2187, 69984, 900000, 6834375, 36756909, 154893312, 544195584, 1660753125, 4529559375, 11270993184, 25984569312, 56162893059, 114865340625, 223948800000, 418732867584, 754570223937, 1315900528659, 2228489100000, 3675690900000, 5919736951359, 9329215288437, 14414112912384, 21870000000000, 32633173828125, 47948838540759, 69454679025312, 99282493797984, 140180883946875
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000848";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_848(n)
    }
}

const fn tri_pow_848(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    9 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000848>();
}
