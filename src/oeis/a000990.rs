/// a(n) = 1*n^2 + 9
/// https://oeis.org/A000990

pub struct A000990;

impl crate::traits::IntegerSequence for A000990 {
    const NAME: &str = "a(n) = 1*n^2 + 9";

    const HEAD: &[crate::Value] = &[
        9, 10, 13, 18, 25, 34, 45, 58, 73, 90, 109, 130, 153, 178, 205, 234, 265, 298, 333, 370, 409, 450, 493, 538, 585, 634, 685, 738, 793, 850
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000990";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_990(n)
    }
}

const fn sq_990(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 9 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000990>();
}
