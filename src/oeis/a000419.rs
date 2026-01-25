/// a(n) = 4*11^n
/// https://oeis.org/A000419

pub struct A000419;

impl crate::traits::IntegerSequence for A000419 {
    const NAME: &str = "a(n) = 4*11^n";

    const HEAD: &[crate::Value] = &[
        4, 44, 484, 5324, 58564, 644204, 7086244, 77948684, 857435524, 9431790764, 103749698404, 1141246682444, 12553713506884, 138090848575724, 1518999334332964, 16708992677662604, 183798919454288644
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000419";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_419(n)
    }
}

const fn pow_419(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 11;
        i += 1;
    }
    4 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000419>();
}
