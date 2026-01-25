/// a(n) = 6*5^n
/// https://oeis.org/A000427

pub struct A000427;

impl crate::traits::IntegerSequence for A000427 {
    const NAME: &str = "a(n) = 6*5^n";

    const HEAD: &[crate::Value] = &[
        6, 30, 150, 750, 3750, 18750, 93750, 468750, 2343750, 11718750, 58593750, 292968750, 1464843750, 7324218750, 36621093750, 183105468750, 915527343750, 4577636718750, 22888183593750, 114440917968750, 572204589843750, 2861022949218750, 14305114746093750, 71525573730468750, 357627868652343750
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000427";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_427(n)
    }
}

const fn pow_427(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 5;
        i += 1;
    }
    6 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000427>();
}
