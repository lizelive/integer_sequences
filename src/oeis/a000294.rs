/// a(n) = 5*n^6
/// https://oeis.org/A000294

pub struct A000294;

impl crate::traits::IntegerSequence for A000294 {
    const NAME: &str = "a(n) = 5*n^6";

    const HEAD: &[crate::Value] = &[
        0, 5, 320, 3645, 20480, 78125, 233280, 588245, 1310720, 2657205, 5000000, 8857805, 14929920, 24134045, 37647680, 56953125, 83886080, 120687845, 170061120, 235229405, 320000000, 428830605, 566899520, 740179445, 955514880
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000294";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_294(n)
    }
}

const fn power_294(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 6 {
        result *= n;
        i += 1;
    }
    5 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000294>();
}
