/// a(n) = 8*3^n
/// https://oeis.org/A000436

pub struct A000436;

impl crate::traits::IntegerSequence for A000436 {
    const NAME: &str = "a(n) = 8*3^n";

    const HEAD: &[crate::Value] = &[
        8, 24, 72, 216, 648, 1944, 5832, 17496, 52488, 157464, 472392, 1417176, 4251528, 12754584, 38263752, 114791256, 344373768, 1033121304, 3099363912, 9298091736, 27894275208, 83682825624, 251048476872, 753145430616, 2259436291848
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000436";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_436(n)
    }
}

const fn pow_436(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 3;
        i += 1;
    }
    8 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000436>();
}
