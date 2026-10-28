/// In-memory state used by the showcase.
#[derive(Default)]
pub struct Counter {
    value: u64,
}

impl Counter {
    /// Returns the current value.
    pub fn value(&self) -> u64 {
        self.value
    }

    /// Increments the value, retaining the maximum when it is reached.
    pub fn increment(&mut self) {
        self.value = self.value.saturating_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::Counter;

    #[test]
    fn increments_from_zero() {
        let mut counter = Counter::default();
        assert_eq!(counter.value(), 0);
        counter.increment();
        counter.increment();
        assert_eq!(counter.value(), 2);
    }

    #[test]
    fn increment_at_maximum_does_not_wrap() {
        let mut counter = Counter { value: u64::MAX };
        counter.increment();
        assert_eq!(counter.value(), u64::MAX);
    }
}
