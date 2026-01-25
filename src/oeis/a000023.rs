/// Expansion of e^(-x)/(1-x).
/// https://oeis.org/A000023

pub struct A000023;

impl crate::traits::IntegerSequence for A000023 {
    const NAME: &str = "Expansion of e^(-x)/(1-x)";

    const HEAD: &[crate::Value] = &[
        1, 0, 1, 2, 9, 44, 265, 1854, 14833, 133496, 1334961, 14684570, 176214841, 2290792932,
        32071101049, 481066515734, 7697064251745, 130850092279664, 2354301661033953,
        44731931559645106,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000023";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        0
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000023>();
}
