/// a(n) = 4*5^n
/// https://oeis.org/A000417

pub struct A000417;

impl crate::traits::IntegerSequence for A000417 {
    const NAME: &str = "a(n) = 4*5^n";

    const HEAD: &[crate::Value] = &[
        4, 20, 100, 500, 2500, 12500, 62500, 312500, 1562500, 7812500, 39062500, 195312500, 976562500, 4882812500, 24414062500, 122070312500, 610351562500, 3051757812500, 15258789062500, 76293945312500, 381469726562500, 1907348632812500, 9536743164062500, 47683715820312500, 238418579101562500
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000417";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_417(n)
    }
}

const fn pow_417(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 5;
        i += 1;
    }
    4 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000417>();
}
