/// a(n) = 3^n.
/// https://oeis.org/A000244

pub struct A000244;

impl crate::traits::IntegerSequence for A000244 {
    const NAME: &str = "Powers of 3: a(n) = 3^n";

    const HEAD: &[crate::Value] = &[
        1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147, 531441, 1594323, 4782969,
        14348907, 43046721, 129140163, 387420489, 1162261467, 3486784401, 10460353203, 31381059609,
        94143178827, 282429536481, 847288609443, 2541865828329, 7625597484987,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000244";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        power_of_3(n)
    }
}

const fn power_of_3(n: crate::Index) -> crate::Value {
    if n < 0 {
        return 0;
    }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 3;
        i += 1;
    }
    result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000244>();
}
