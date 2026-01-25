/// Number of partitions of n if there are two kinds of 1's, 2's, and 3's.
/// https://oeis.org/A000104
/// 
/// G.f.: 1/((1-x)^2 * (1-x^2)^2 * (1-x^3)^2 * (1-x^4) * (1-x^5) * ...)

pub struct A000104;

impl crate::traits::IntegerSequence for A000104 {
    const NAME: &str = "Number of partitions of n if there are two kinds of 1's, 2's, and 3's";

    const HEAD: &[crate::Value] = &[
        1, 2, 5, 10, 19, 33, 57, 92, 147, 227, 345, 512, 752, 1083, 1545, 2174, 3031, 4179, 5719,
        7752, 10438, 13946, 18519, 24428, 32051, 41805, 54265, 70079, 90102, 115318, 147005,
        186626, 236064, 297492, 373645, 467707, 583644, 726058, 900644, 1114027, 1374303,
        1690918, 2075337, 2540903, 3103729, 3782572, 4599907, 5581911, 6759796, 8169834,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000104";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        partitions_two_kinds_1_2_3(n)
    }
}

fn partitions_two_kinds_1_2_3(n: crate::Index) -> crate::Value {
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
    
    // Apply 1/(1-x^2)^2
    let mut pass = 0;
    while pass < 2 {
        let mut i = 2usize;
        while i <= n_usize {
            a[i] += a[i - 2];
            i += 1;
        }
        pass += 1;
    }
    
    // Apply 1/(1-x^3)^2
    let mut pass = 0;
    while pass < 2 {
        let mut i = 3usize;
        while i <= n_usize {
            a[i] += a[i - 3];
            i += 1;
        }
        pass += 1;
    }
    
    // Apply 1/(1-x^k) for k >= 4
    let mut k = 4usize;
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
    crate::tester::test_sequance_formula_matchces_head::<A000104>();
}
