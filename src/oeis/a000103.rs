/// Number of partitions of n if there are two kinds of 1's and 2's.
/// https://oeis.org/A000103
/// 
/// G.f.: 1/((1-x)^2 * (1-x^2)^2 * (1-x^3) * (1-x^4) * ...)

pub struct A000103;

impl crate::traits::IntegerSequence for A000103 {
    const NAME: &str = "Number of partitions of n if there are two kinds of 1's and 2's";

    const HEAD: &[crate::Value] = &[
        1, 2, 5, 9, 17, 28, 47, 73, 114, 170, 253, 365, 525, 738, 1033, 1422, 1948, 2634, 3545,
        4721, 6259, 8227, 10767, 13990, 18105, 23286, 29837, 38028, 48297, 61053, 76926, 96524,
        120746, 150487, 187019, 231643, 286152, 352413, 432937, 530383, 648245, 790274, 961310,
        1166600, 1412811, 1707235, 2059004, 2478182, 2977224, 3569927,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000103";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        partitions_two_kinds_1_and_2(n)
    }
}

fn partitions_two_kinds_1_and_2(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    
    const MAX: usize = 500;
    let n_usize = n as usize;
    if n_usize >= MAX { return 0; }
    
    let mut a = [0isize; MAX];
    let mut i = 0usize;
    while i <= n_usize {
        a[i] = (i + 1) as isize;
        i += 1;
    }
    
    // Apply 1/(1-x^2)^2 = apply 1/(1-x^2) twice
    let mut pass = 0;
    while pass < 2 {
        let mut i = 2usize;
        while i <= n_usize {
            a[i] += a[i - 2];
            i += 1;
        }
        pass += 1;
    }
    
    // Apply 1/(1-x^k) for k >= 3
    let mut k = 3usize;
    while k <= n_usize {
        let mut i = k;
        while i <= n_usize {
            a[i] += a[i - k];
            i += 1;
        }
        k += 1;
    }
    
    a[n_usize]
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000103>();
}
