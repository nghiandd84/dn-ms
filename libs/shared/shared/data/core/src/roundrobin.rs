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

