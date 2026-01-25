/// a(n) = 9*11^n
/// https://oeis.org/A000444

pub struct A000444;

impl crate::traits::IntegerSequence for A000444 {
    const NAME: &str = "a(n) = 9*11^n";

    const HEAD: &[crate::Value] = &[
        9, 99, 1089, 11979, 131769, 1449459, 15944049, 175384539, 1929229929, 21221529219, 233436821409, 2567805035499, 28245855390489, 310704409295379, 3417748502249169, 37595233524740859, 413547568772149449
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000444";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        pow_444(n)
    }
}

const fn pow_444(n: crate::Index) -> crate::Value {
    if n < 0 || n > 40 { return 0; }
    let mut result = 1isize;
    let mut i = 0;
    while i < n {
        result *= 11;
        i += 1;
    }
    9 * result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000444>();
}
