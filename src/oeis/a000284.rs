/// a(n) = 5*n^5
/// https://oeis.org/A000284

pub struct A000284;

impl crate::traits::IntegerSequence for A000284 {
    const NAME: &str = "a(n) = 5*n^5";

    const HEAD: &[crate::Value] = &[
        0, 5, 160, 1215, 5120, 15625, 38880, 84035, 163840, 295245, 500000, 805255, 1244160, 1856465, 2689120, 3796875, 5242880, 7099285, 9447840, 12380495, 16000000, 20420505, 25768160, 32181715, 39813120
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000284";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_284(n)
    }
}

const fn power_284(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 5 {
        result *= n;
        i += 1;
    }
    5 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000284>();
}
