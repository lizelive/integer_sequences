/// a(n) = n^3 + 7*n + 2
/// https://oeis.org/A000527

pub struct A000527;

impl crate::traits::IntegerSequence for A000527 {
    const NAME: &str = "a(n) = n^3 + 7*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 10, 24, 50, 94, 162, 260, 394, 570, 794, 1072, 1410, 1814, 2290, 2844, 3482, 4210, 5034, 5960, 6994, 8142, 9410, 10804, 12330, 13994, 15802, 17760, 19874, 22150, 24594
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000527";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_527(n)
    }
}

const fn poly_527(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 7 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000527>();
}
