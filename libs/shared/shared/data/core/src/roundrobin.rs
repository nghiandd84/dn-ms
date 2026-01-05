/// A struct to manage the round-robin state.
/// It is now generic over the type `T`.
#[derive(Debug, Clone)]
pub struct RoundRobin<T> {
    values: Vec<T>,
    current_index: usize,
}

impl<T> RoundRobin<T> {
    /// Creates a new `RoundRobin` instance with a vector of generic values.
    pub fn new(values: Vec<T>) -> Self {
        RoundRobin {
            values,
            current_index: 0,
        }
    }

    /// Returns a reference to the next value in the sequence.
    /// The function mutates the internal state to move to the next index.
    pub fn next_value(&mut self) -> &T {
        // Handle the case where the values vector is empty to prevent a panic
        // This check is important as it provides a safe return value for empty lists.
        if self.values.is_empty() {
            panic!("Cannot call next_value on an empty RoundRobin generator.");
        }

        // Get the value at the current index
        let value = &self.values[self.current_index];

        // Increment the index and wrap around to 0 if we reach the end of the vector
        self.current_index = (self.current_index + 1) % self.values.len();

        value
    }

    /// Replaces all the values in the RoundRobin generator with a new set of values.
    /// The current index is reset to 0.
    pub fn replace_values(&mut self, new_values: Vec<T>) {
        if self.values.len() != (new_values.len()) {
            self.current_index = 0;
        }
        self.values = new_values;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_value_cycles_through_values() {
        let mut rr = RoundRobin::new(vec![1, 2, 3]);
        assert_eq!(*rr.next_value(), 1);
        assert_eq!(*rr.next_value(), 2);
        assert_eq!(*rr.next_value(), 3);
        assert_eq!(*rr.next_value(), 1); // Should cycle back
    }

    #[test]
    fn test_next_value_with_strings() {
        let mut rr = RoundRobin::new(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(rr.next_value(), "a");
        assert_eq!(rr.next_value(), "b");
        assert_eq!(rr.next_value(), "a");
    }

    #[test]
    #[should_panic(expected = "Cannot call next_value on an empty RoundRobin generator.")]
    fn test_next_value_empty_panics() {
        let mut rr: RoundRobin<i32> = RoundRobin::new(vec![]);
        rr.next_value();
    }

    #[test]
    fn test_replace_values_resets_index() {
        let mut rr = RoundRobin::new(vec![1, 2, 3]);
        rr.next_value(); // index = 1
        rr.replace_values(vec![10, 20]);
        assert_eq!(*rr.next_value(), 10);
        assert_eq!(*rr.next_value(), 20);
    }

    #[test]
    fn test_replace_values_with_empty() {
        let mut rr = RoundRobin::new(vec![1, 2]);
        assert_eq!(*rr.next_value(), 1);
        rr.replace_values(vec![]);
        assert_eq!(rr.values.len(), 0);
        assert_eq!(rr.current_index, 0);
    }
}
