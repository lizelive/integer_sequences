/// Number of partitions of n if there are two kinds of 1's.
/// https://oeis.org/A000102
/// 
/// G.f.: 1/((1-x)^2 * (1-x^2) * (1-x^3) * ...)
/// a(n) = sum_{k=0}^n (k+1) * p2(n-k) where p2(m) = partitions of m into parts >= 2

pub struct A000102;

impl crate::traits::IntegerSequence for A000102 {
    const NAME: &str = "Number of partitions of n if there are two kinds of 1's";

    const HEAD: &[crate::Value] = &[
        1, 2, 4, 7, 12, 19, 30, 45, 67, 97, 139, 195, 272, 373, 508, 684, 915, 1212, 1597, 2087,
        2714, 3506, 4508, 5763, 7338, 9296, 11732, 14742, 18460, 23025, 28629, 35471, 43820,
        53963, 66273, 81156, 99133, 120770, 146785, 177970, 215308, 259891, 313065, 376326,
        451501, 540635, 646193, 770947, 918220, 1091745,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000102";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        partitions_two_kinds_1(n)
    }
}

fn partitions_two_kinds_1(n: crate::Index) -> crate::Value {
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
    
    let mut k = 2usize;
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
    crate::tester::test_sequance_formula_matchces_head::<A000102>();
}
