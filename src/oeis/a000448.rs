/// a(n) = 10*7^n
/// https://oeis.org/A000448

pub struct A000448;

impl crate::traits::IntegerSequence for A000448 {
    const NAME: &str = "a(n) = 10*7^n";

    const HEAD: &[crate::Value] = &[
        10, 70, 490, 3430, 24010, 168070, 1176490, 8235430, 57648010, 403536070, 2824752490, 19773267430, 138412872010, 968890104070, 6782230728490, 47475615099430, 332329305696010, 2326305139872070, 16284135979104490, 113988951853731430, 797922662976120010
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000448";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_448(n)
    }
}

const fn pow_448(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 7;
        i += 1;
    }
    10 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000448>();
}
