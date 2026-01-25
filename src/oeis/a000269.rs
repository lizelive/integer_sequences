/// a(n) = 10*n^3
/// https://oeis.org/A000269

pub struct A000269;

impl crate::traits::IntegerSequence for A000269 {
    const NAME: &str = "a(n) = 10*n^3";

    const HEAD: &[crate::Value] = &[
        0, 10, 80, 270, 640, 1250, 2160, 3430, 5120, 7290, 10000, 13310, 17280, 21970, 27440, 33750, 40960, 49130, 58320, 68590, 80000, 92610, 106480, 121670, 138240
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000269";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_269(n)
    }
}

const fn power_269(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 3 {
        result *= n;
        i += 1;
    }
    10 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000269>();
}
