/// a(n) = 10*3^n
/// https://oeis.org/A000446

pub struct A000446;

impl crate::traits::IntegerSequence for A000446 {
    const NAME: &str = "a(n) = 10*3^n";

    const HEAD: &[crate::Value] = &[
        10, 30, 90, 270, 810, 2430, 7290, 21870, 65610, 196830, 590490, 1771470, 5314410, 15943230, 47829690, 143489070, 430467210, 1291401630, 3874204890, 11622614670, 34867844010, 104603532030, 313810596090, 941431788270, 2824295364810
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000446";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_446(n)
    }
}

const fn pow_446(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 3;
        i += 1;
    }
    10 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000446>();
}
