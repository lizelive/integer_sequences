/// a(n) = n! + 1
/// https://oeis.org/A000141

pub struct A000141;

impl crate::traits::IntegerSequence for A000141 {
    const NAME: &str = "a(n) = n! + 1";

    const HEAD: &[crate::Value] = &[
        2, 2, 3, 7, 25, 121, 721, 5041, 40321, 362881, 3628801, 39916801, 479001601, 6227020801, 87178291201, 1307674368001, 20922789888001, 355687428096001, 6402373705728001, 121645100408832001
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000141";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        factorial_plus_1(n)
    }
}

fn factorial_plus_1(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = 1isize;
    let mut i = 2isize;
    while i <= n {
        result = result.saturating_mul(i);
        i += 1;
    }
    result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000141>();
}
