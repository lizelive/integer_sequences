/// a(n) = 1*n^5
/// https://oeis.org/A000280

pub struct A000280;

impl crate::traits::IntegerSequence for A000280 {
    const NAME: &str = "a(n) = 1*n^5";

    const HEAD: &[crate::Value] = &[
        0, 1, 32, 243, 1024, 3125, 7776, 16807, 32768, 59049, 100000, 161051, 248832, 371293, 537824, 759375, 1048576, 1419857, 1889568, 2476099, 3200000, 4084101, 5153632, 6436343, 7962624
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000280";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        power_280(n)
    }
}

const fn power_280(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let mut result = n;
    let mut i = 1;
    while i < 5 {
        result *= n;
        i += 1;
    }
    1 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000280>();
}
