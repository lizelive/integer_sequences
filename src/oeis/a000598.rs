/// a(n) = n^3 + 8*n + 9
/// https://oeis.org/A000598

pub struct A000598;

impl crate::traits::IntegerSequence for A000598 {
    const NAME: &str = "a(n) = n^3 + 8*n + 9";

    const HEAD: &[crate::Value] = &[
        9, 18, 33, 60, 105, 174, 273, 408, 585, 810, 1089, 1428, 1833, 2310, 2865, 3504, 4233, 5058, 5985, 7020, 8169, 9438, 10833, 12360, 14025, 15834, 17793, 19908, 22185, 24630
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000598";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_598(n)
    }
}

const fn poly_598(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 8 * n + 9
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000598>();
}
