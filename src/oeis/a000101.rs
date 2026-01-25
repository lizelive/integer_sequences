/// Increasing gaps between primes (upper end): a(n) = A000040(A002386(n)+1).
/// https://oeis.org/A000101

pub struct A000101;

impl crate::traits::IntegerSequence for A000101 {
    const NAME: &str = "Increasing gaps between primes (upper end)";

    const HEAD: &[crate::Value] = &[
        3, 5, 11, 29, 97, 127, 541, 907, 1151, 1361, 9587, 15727, 19661, 31469, 156007, 360749,
        370373, 492227, 1349651, 1357333, 2010881, 4652507, 17051887, 20831533, 47326913,
        122164969, 189695893, 191913031,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000101";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        0
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000101>();
}
