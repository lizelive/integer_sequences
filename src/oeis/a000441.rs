/// a(n) = 9*3^n
/// https://oeis.org/A000441

pub struct A000441;

impl crate::traits::IntegerSequence for A000441 {
    const NAME: &str = "a(n) = 9*3^n";

    const HEAD: &[crate::Value] = &[
        9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147, 531441, 1594323, 4782969, 14348907, 43046721, 129140163, 387420489, 1162261467, 3486784401, 10460353203, 31381059609, 94143178827, 282429536481, 847288609443, 2541865828329
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000441";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_441(n)
    }
}

const fn pow_441(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 3;
        i += 1;
    }
    9 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000441>();
}
