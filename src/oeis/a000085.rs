/// Number of self-inverse permutations on n elements, also known as involutions; number of standard Young tableaux with n cells.
/// https://oeis.org/A000085

pub struct A000085;

impl crate::traits::IntegerSequence for A000085 {
    const NAME: &str = "Number of involutions on n elements";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 4, 10, 26, 76, 232, 764, 2620, 9496, 35696, 140152, 568504, 2390480, 10349536,
        46206736, 211799312, 997313824, 4809701440, 23758664096, 119952692896, 618884638912,
        3257843882624, 17492190577600, 95680443760576,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000085";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        involutions(n)
    }
}

const fn involutions(n: crate::Index) -> crate::Value {
    if n < 0 {
        return 0;
    }
    if n <= 1 {
        return 1;
    }
    
    const MAX_N: usize = 100;
    let n = n as usize;
    if n >= MAX_N {
        return 0;
    }
    
    let mut a = [0isize; MAX_N];
    a[0] = 1;
    a[1] = 1;
    
    let mut i = 2usize;
    while i <= n {
        a[i] = a[i - 1] + (i as isize - 1) * a[i - 2];
        i += 1;
    }
    a[n]
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000085>();
}
