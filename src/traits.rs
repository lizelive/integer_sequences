pub type Index = isize;

pub type Value = isize;

/// all the traits would be const if it where allowed in stable rust
pub trait IntegerSequence {
    /// the name
    const NAME: &str;

    /// the first values
    const HEAD: &[Value];

    const OFFSET: Index;

    /// source url
    /// example : https://oeis.org/A000001
    const SOURCE: &str;

    /// who provided this
    const AUTHOR: &str;

    /// static impl
    fn formula(n: Index) -> Value;

    // get
    fn get_head(n: Index) -> Option<Value> {
        let head_index: usize = (n - Self::OFFSET).try_into().ok()?;
        let head_value = Self::HEAD.get(head_index)?;
        Some(*head_value)
    }

    fn call(n: Index) -> Value {
        Self::get_head(n).unwrap_or_else(|| Self::formula(n))
    }
}
