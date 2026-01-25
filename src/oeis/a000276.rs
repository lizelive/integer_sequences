/// a(n) = 7*n^4
/// https://oeis.org/A000276

pub struct A000276;

impl crate::traits::IntegerSequence for A000276 {
    const NAME: &str = "a(n) = 7*n^4";

    const HEAD: &[crate::Value] = &[
        0, 7, 112, 567, 1792, 4375, 9072, 16807, 28672, 45927, 70000, 102487, 145152, 199927, 268912, 354375, 458752, 584647, 734832, 912247, 1120000, 1361367, 1639792, 1958887, 2322432
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000276";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_276(n)
    }
}

const fn power_276(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 4 {
        result *= n;
        i += 1;
    }
    7 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000276>();
}
