/// a(n) = 4*T(n)^2 + 1
/// https://oeis.org/A000863

pub struct A000863;

impl crate::traits::IntegerSequence for A000863 {
    const NAME: &str = "a(n) = 4*T(n)^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 5, 37, 145, 401, 901, 1765, 3137, 5185, 8101, 12101, 17425, 24337, 33125, 44101, 57601, 73985, 93637, 116965, 144401, 176401, 213445, 256037, 304705, 360001, 422501, 492805, 571537, 659345, 756901
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000863";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_863(n)
    }
}

const fn tri_pow_863(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 2 {
        result *= t;
        i += 1;
    }
    4 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000863>();
}
