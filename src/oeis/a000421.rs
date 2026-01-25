/// a(n) = 5*3^n
/// https://oeis.org/A000421

pub struct A000421;

impl crate::traits::IntegerSequence for A000421 {
    const NAME: &str = "a(n) = 5*3^n";

    const HEAD: &[crate::Value] = &[
        5, 15, 45, 135, 405, 1215, 3645, 10935, 32805, 98415, 295245, 885735, 2657205, 7971615, 23914845, 71744535, 215233605, 645700815, 1937102445, 5811307335, 17433922005, 52301766015, 156905298045, 470715894135, 1412147682405
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000421";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_421(n)
    }
}

const fn pow_421(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 3;
        i += 1;
    }
    5 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000421>();
}
