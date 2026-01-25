/// a(n) = 8*T(n)^3 + 1
/// https://oeis.org/A000877

pub struct A000877;

impl crate::traits::IntegerSequence for A000877 {
    const NAME: &str = "a(n) = 8*T(n)^3 + 1";

    const HEAD: &[crate::Value] = &[
        1, 9, 217, 1729, 8001, 27001, 74089, 175617, 373249, 729001, 1331001, 2299969, 3796417, 6028569, 9261001, 13824001, 20123649, 28652617, 40001689, 54872001, 74088001, 98611129, 129554217, 168196609, 216000001, 274625001, 345948409, 432081217, 535387329, 658503001
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000877";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_877(n)
    }
}

const fn tri_pow_877(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 3 {
        result *= t;
        i += 1;
    }
    8 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000877>();
}
