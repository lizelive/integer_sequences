/// a(n) = 10*5^n
/// https://oeis.org/A000447

pub struct A000447;

impl crate::traits::IntegerSequence for A000447 {
    const NAME: &str = "a(n) = 10*5^n";

    const HEAD: &[crate::Value] = &[
        10, 50, 250, 1250, 6250, 31250, 156250, 781250, 3906250, 19531250, 97656250, 488281250, 2441406250, 12207031250, 61035156250, 305175781250, 1525878906250, 7629394531250, 38146972656250, 190734863281250, 953674316406250, 4768371582031250, 23841857910156250, 119209289550781250, 596046447753906250
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000447";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_447(n)
    }
}

const fn pow_447(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 5;
        i += 1;
    }
    10 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000447>();
}
