/// a(n) = 10*T(n)^4
/// https://oeis.org/A000839

pub struct A000839;

impl crate::traits::IntegerSequence for A000839 {
    const NAME: &str = "a(n) = 10*T(n)^4";

    const HEAD: &[crate::Value] = &[
        0, 10, 810, 12960, 100000, 506250, 1944810, 6146560, 16796160, 41006250, 91506250, 189747360, 370150560, 685749610, 1215506250, 2073600000, 3421020160, 5479812810, 8550360810, 13032100000, 19448100000, 28473963210, 40971520810, 58027829760, 81000000000, 111566406250, 151784864010, 204158374560, 271709068960, 358061006250
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000839";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_839(n)
    }
}

const fn tri_pow_839(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    10 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000839>();
}
