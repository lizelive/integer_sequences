/// a(n) = 6*n^6
/// https://oeis.org/A000295

pub struct A000295;

impl crate::traits::IntegerSequence for A000295 {
    const NAME: &str = "a(n) = 6*n^6";

    const HEAD: &[crate::Value] = &[
        0, 6, 384, 4374, 24576, 93750, 279936, 705894, 1572864, 3188646, 6000000, 10629366, 17915904, 28960854, 45177216, 68343750, 100663296, 144825414, 204073344, 282275286, 384000000, 514596726, 680279424, 888215334, 1146617856
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000295";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_295(n)
    }
}

const fn power_295(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 6 {
        result *= n;
        i += 1;
    }
    6 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000295>();
}
