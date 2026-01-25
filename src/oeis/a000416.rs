/// a(n) = 4*3^n
/// https://oeis.org/A000416

pub struct A000416;

impl crate::traits::IntegerSequence for A000416 {
    const NAME: &str = "a(n) = 4*3^n";

    const HEAD: &[crate::Value] = &[
        4, 12, 36, 108, 324, 972, 2916, 8748, 26244, 78732, 236196, 708588, 2125764, 6377292, 19131876, 57395628, 172186884, 516560652, 1549681956, 4649045868, 13947137604, 41841412812, 125524238436, 376572715308, 1129718145924
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000416";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_416(n)
    }
}

const fn pow_416(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 3;
        i += 1;
    }
    4 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000416>();
}
