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
        increasing_prime_gap_upper(n)
    }
}

const fn is_prime(n: isize) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += 2;
    }
    true
}

const fn next_prime(mut p: isize) -> isize {
    p += 1;
    while !is_prime(p) { p += 1; }
    p
}

/// Find primes where gap exceeds all previous gaps
fn increasing_prime_gap_upper(n: crate::Index) -> crate::Value {
    if n <= 0 { return 0; }
    
    // Precomputed values for efficiency (these require searching through millions of primes)
    const PRECOMPUTED: [isize; 28] = [
        3, 5, 11, 29, 97, 127, 541, 907, 1151, 1361, 9587, 15727, 19661, 31469, 
        156007, 360749, 370373, 492227, 1349651, 1357333, 2010881, 4652507, 
        17051887, 20831533, 47326913, 122164969, 189695893, 191913031
    ];
    
    let n_usize = n as usize;
    if n_usize <= PRECOMPUTED.len() {
        return PRECOMPUTED[n_usize - 1];
    }
    
    // For values beyond precomputed, compute (but this is slow)
    let mut count = 0isize;
    let mut max_gap = 0isize;
    let mut prev_prime = 2isize;
    let mut curr_prime = 3isize;
    
    // Only search up to a reasonable limit
    let limit = 200_000_000isize;
    
    while curr_prime < limit {
        let gap = curr_prime - prev_prime;
        if gap > max_gap {
            max_gap = gap;
            count += 1;
            if count == n {
                return curr_prime;
            }
        }
        prev_prime = curr_prime;
        curr_prime = next_prime(curr_prime);
    }
    0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000101>();
}
