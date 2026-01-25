/// a(n) = 9*7^n
/// https://oeis.org/A000443

pub struct A000443;

impl crate::traits::IntegerSequence for A000443 {
    const NAME: &str = "a(n) = 9*7^n";

    const HEAD: &[crate::Value] = &[
        9, 63, 441, 3087, 21609, 151263, 1058841, 7411887, 51883209, 363182463, 2542277241, 17795940687, 124571584809, 872001093663, 6104007655641, 42728053589487, 299096375126409, 2093674625884863, 14655722381194041, 102590056668358287, 718130396678508009
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000443";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_443(n)
    }
}

const fn pow_443(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 7;
        i += 1;
    }
    9 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000443>();
}
