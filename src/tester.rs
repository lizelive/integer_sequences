pub fn test_sequance_formula_matchces_head<T: crate::IntegerSequence>() {
    T::HEAD.iter().cloned().enumerate().for_each(|(i, actual)| {
        let n = i as crate::Value + T::OFFSET;
        let computed = T::formula(n);
        assert_eq!(computed, actual, "a({}) = {}, got {}", n, actual, computed);
    });
}
