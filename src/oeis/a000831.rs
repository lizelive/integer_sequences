/// a(n) = 2*T(n)^4
/// https://oeis.org/A000831

pub struct A000831;

impl crate::traits::IntegerSequence for A000831 {
    const NAME: &str = "a(n) = 2*T(n)^4";

    const HEAD: &[crate::Value] = &[
        0, 2, 162, 2592, 20000, 101250, 388962, 1229312, 3359232, 8201250, 18301250, 37949472, 74030112, 137149922, 243101250, 414720000, 684204032, 1095962562, 1710072162, 2606420000, 3889620000, 5694792642, 8194304162, 11605565952, 16200000000, 22313281250, 30356972802, 40831674912, 54341813792, 71612201250
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000831";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_831(n)
    }
}

const fn tri_pow_831(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    2 * result + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000831>();
}
