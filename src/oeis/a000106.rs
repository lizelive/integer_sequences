/// Second power of triangular numbers: a(n) = (n*(n+1)/2)^2.
/// https://oeis.org/A000106

pub struct A000106;

impl crate::traits::IntegerSequence for A000106 {
    const NAME: &str = "2nd power of triangular numbers";

    const HEAD: &[crate::Value] = &[
        0, 1, 9, 36, 100, 225, 441, 784, 1296, 2025, 3025, 4356, 6084, 8281, 11025, 14400, 18496, 23409, 29241, 36100, 44100, 53361, 64009, 76176, 90000, 105625, 123201, 142884, 164836, 189225, 216225, 246016, 278784, 314721, 354025, 396900, 443556, 494209, 549081, 608400
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000106";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        triangular_squared(n)
    }
}

const fn triangular_squared(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    t * t
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000106>();
}
