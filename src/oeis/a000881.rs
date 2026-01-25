/// a(n) = 2*T(n)^4 + 1
/// https://oeis.org/A000881

pub struct A000881;

impl crate::traits::IntegerSequence for A000881 {
    const NAME: &str = "a(n) = 2*T(n)^4 + 1";

    const HEAD: &[crate::Value] = &[
        1, 3, 163, 2593, 20001, 101251, 388963, 1229313, 3359233, 8201251, 18301251, 37949473, 74030113, 137149923, 243101251, 414720001, 684204033, 1095962563, 1710072163, 2606420001, 3889620001, 5694792643, 8194304163, 11605565953, 16200000001, 22313281251, 30356972803, 40831674913, 54341813793, 71612201251
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000881";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_pow_881(n)
    }
}

const fn tri_pow_881(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    let t = n * (n + 1) / 2;
    let mut result = t;
    let mut i = 1;
    while i < 4 {
        result *= t;
        i += 1;
    }
    2 * result + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000881>();
}
