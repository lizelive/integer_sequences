/// a(n) = 7*3^n
/// https://oeis.org/A000431

pub struct A000431;

impl crate::traits::IntegerSequence for A000431 {
    const NAME: &str = "a(n) = 7*3^n";

    const HEAD: &[crate::Value] = &[
        7, 21, 63, 189, 567, 1701, 5103, 15309, 45927, 137781, 413343, 1240029, 3720087, 11160261, 33480783, 100442349, 301327047, 903981141, 2711943423, 8135830269, 24407490807, 73222472421, 219667417263, 659002251789, 1977006755367
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000431";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_431(n)
    }
}

const fn pow_431(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 3;
        i += 1;
    }
    7 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000431>();
}
