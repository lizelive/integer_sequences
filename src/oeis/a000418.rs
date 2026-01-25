/// a(n) = 4*7^n
/// https://oeis.org/A000418

pub struct A000418;

impl crate::traits::IntegerSequence for A000418 {
    const NAME: &str = "a(n) = 4*7^n";

    const HEAD: &[crate::Value] = &[
        4, 28, 196, 1372, 9604, 67228, 470596, 3294172, 23059204, 161414428, 1129900996, 7909306972, 55365148804, 387556041628, 2712892291396, 18990246039772, 132931722278404, 930522055948828, 6513654391641796, 45595580741492572, 319169065190448004
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000418";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_418(n)
    }
}

const fn pow_418(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 7;
        i += 1;
    }
    4 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000418>();
}
