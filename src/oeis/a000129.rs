/// Pell numbers: a(0) = 0, a(1) = 1; a(n) = 2*a(n-1) + a(n-2)
/// https://oeis.org/A000129

pub struct A000129;

impl crate::traits::IntegerSequence for A000129 {
    const NAME: &str = "Pell numbers";

    const HEAD: &[crate::Value] = &[
        0, 1, 2, 5, 12, 29, 70, 169, 408, 985, 2378, 5741, 13860, 33461, 80782, 195025, 470832, 1136689, 2744210, 6625109, 15994428, 38613965, 93222358, 225058681, 543339720, 1311738121, 3166815962, 7645370045, 18457556052, 44560482149
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000129";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pell(n)
    }
}

fn pell(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    
    let mut a = 0isize;
    let mut b = 1isize;
    let mut i = 2isize;
    while i <= n {
        let c = 2 * b + a;
        a = b;
        b = c;
        i += 1;
    }
    b
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000129>();
}
