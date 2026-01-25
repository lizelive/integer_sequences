/// a(n) = n^3 + 4*n + 9
/// https://oeis.org/A000594

pub struct A000594;

impl crate::traits::IntegerSequence for A000594 {
    const NAME: &str = "a(n) = n^3 + 4*n + 9";

    const HEAD: &[crate::Value] = &[
        9, 14, 25, 48, 89, 154, 249, 380, 553, 774, 1049, 1384, 1785, 2258, 2809, 3444, 4169, 4990, 5913, 6944, 8089, 9354, 10745, 12268, 13929, 15734, 17689, 19800, 22073, 24514
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000594";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_594(n)
    }
}

const fn poly_594(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n + 9
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000594>();
}
