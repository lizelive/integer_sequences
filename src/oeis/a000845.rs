/// a(n) = 6*T(n)^5
/// https://oeis.org/A000845

pub struct A000845;

impl crate::traits::IntegerSequence for A000845 {
    const NAME: &str = "a(n) = 6*T(n)^5";

    const HEAD: &[crate::Value] = &[
        0, 6, 1458, 46656, 600000, 4556250, 24504606, 103262208, 362797056, 1107168750, 3019706250, 7513995456, 17323046208, 37441928706, 76576893750, 149299200000, 279155245056, 503046815958, 877267019106, 1485659400000, 2450460600000, 3946491300906, 6219476858958, 9609408608256, 14580000000000, 21755449218750, 31965892360506, 46303119350208, 66188329198656, 93453922631250
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000845";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_845(n)
    }
}

const fn tri_pow_845(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 5 {
        result *= t;
        i += 1;
    }
    6 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000845>();
}
