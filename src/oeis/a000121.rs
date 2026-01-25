/// Subfactorials
/// https://oeis.org/A000121

pub struct A000121;

impl crate::traits::IntegerSequence for A000121 {
    const NAME: &str = "a(n) = n! * Sum_{k=0..n} 1/k!";

    const HEAD: &[crate::Value] = &[
        1, 2, 5, 16, 65, 326, 1957, 13700, 109601, 986410, 9864101, 108505112, 1302061345
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000121";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        a121_formula(n)
    }
}

fn a121_formula(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    if n == 0 { return 1; }
    
    // a(n) = n * a(n-1) + 1
    let mut result = 1isize;
    for k in 1..=n {
        result = k * result + 1;
    }
    result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000121>();
}
