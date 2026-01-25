/// a(n) = 5*11^n
/// https://oeis.org/A000424

pub struct A000424;

impl crate::traits::IntegerSequence for A000424 {
    const NAME: &str = "a(n) = 5*11^n";

    const HEAD: &[crate::Value] = &[
        5, 55, 605, 6655, 73205, 805255, 8857805, 97435855, 1071794405, 11789738455, 129687123005, 1426558353055, 15692141883605, 172613560719655, 1898749167916205, 20886240847078255, 229748649317860805
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000424";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_424(n)
    }
}

const fn pow_424(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 11;
        i += 1;
    }
    5 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000424>();
}
