/// Number of binary partitions: partitions of n into powers of 2
/// https://oeis.org/A000123
/// 
/// Recurrence: a(0)=1, a(2n)=a(2n-1)+a(n), a(2n+1)=a(2n)

pub struct A000123;

impl crate::traits::IntegerSequence for A000123 {
    const NAME: &str = "Number of binary partitions";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 2, 4, 4, 6, 6, 10, 10, 14, 14, 20, 20, 26, 26, 36, 36, 46, 46, 60, 60, 74, 74,
        94, 94, 114, 114, 140, 140, 166, 166, 202, 202, 238, 238, 284, 284, 330, 330,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000123";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        binary_partitions(n)
    }
}

fn binary_partitions(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    if n == 0 { return 1; }
    
    const MAX: usize = 1000;
    let n_usize = n as usize;
    if n_usize >= MAX { return 0; }
    
    let mut a = [0isize; MAX];
    a[0] = 1;
    
    let mut i = 1usize;
    while i <= n_usize {
        if i % 2 == 0 {
            a[i] = a[i - 1] + a[i / 2];
        } else {
            a[i] = a[i - 1];
        }
        i += 1;
    }
    a[n_usize]
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000123>();
}
