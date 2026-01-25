/// a(n) = 7*11^n
/// https://oeis.org/A000434

pub struct A000434;

impl crate::traits::IntegerSequence for A000434 {
    const NAME: &str = "a(n) = 7*11^n";

    const HEAD: &[crate::Value] = &[
        7, 77, 847, 9317, 102487, 1127357, 12400927, 136410197, 1500512167, 16505633837, 181561972207, 1997181694277, 21968998637047, 241658985007517, 2658248835082687, 29240737185909557, 321648109045005127
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000434";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_434(n)
    }
}

const fn pow_434(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 11;
        i += 1;
    }
    7 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000434>();
}
