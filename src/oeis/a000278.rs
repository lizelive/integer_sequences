/// a(n) = 9*n^4
/// https://oeis.org/A000278

pub struct A000278;

impl crate::traits::IntegerSequence for A000278 {
    const NAME: &str = "a(n) = 9*n^4";

    const HEAD: &[crate::Value] = &[
        0, 9, 144, 729, 2304, 5625, 11664, 21609, 36864, 59049, 90000, 131769, 186624, 257049, 345744, 455625, 589824, 751689, 944784, 1172889, 1440000, 1750329, 2108304, 2518569, 2985984
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000278";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_278(n)
    }
}

const fn power_278(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 4 {
        result *= n;
        i += 1;
    }
    9 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000278>();
}
