/// a(n) = 1*7^n
/// https://oeis.org/A000403

pub struct A000403;

impl crate::traits::IntegerSequence for A000403 {
    const NAME: &str = "a(n) = 1*7^n";

    const HEAD: &[crate::Value] = &[
        1, 7, 49, 343, 2401, 16807, 117649, 823543, 5764801, 40353607, 282475249, 1977326743, 13841287201, 96889010407, 678223072849, 4747561509943, 33232930569601, 232630513987207, 1628413597910449, 11398895185373143, 79792266297612001, 558545864083284007
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000403";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_403(n)
    }
}

const fn pow_403(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 7;
        i += 1;
    }
    1 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000403>();
}
