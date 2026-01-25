/// a(n) = 2*7^n
/// https://oeis.org/A000408

pub struct A000408;

impl crate::traits::IntegerSequence for A000408 {
    const NAME: &str = "a(n) = 2*7^n";

    const HEAD: &[crate::Value] = &[
        2, 14, 98, 686, 4802, 33614, 235298, 1647086, 11529602, 80707214, 564950498, 3954653486, 27682574402, 193778020814, 1356446145698, 9495123019886, 66465861139202, 465261027974414, 3256827195820898, 22797790370746286, 159584532595224002
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000408";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_408(n)
    }
}

const fn pow_408(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 7;
        i += 1;
    }
    2 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000408>();
}
