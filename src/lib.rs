struct AggregatedStats {
    values: Vec<usize>,
    max_size: usize,
    max: Option<usize>,
    min: Option<usize>,
}

impl AggregatedStats {
    fn new() -> AggregatedStats {
        AggregatedStats {
            values: vec![],
            max_size: 10000,
            max: None,
            min: None,
        }
    }

    fn with_capacity(capacity: usize) -> AggregatedStats {
        AggregatedStats {
            values: vec![],
            max_size: capacity,
            max: None,
            min: None,
        }
    }

    fn add(&mut self, value: usize) {
        if self.values.len() < self.max_size {
            self.values.push(value);
        } else {
            self.values.sort();
            let index = match self.values.binary_search(&value) {
                Ok(index) => index,
                Err(index) => index,
            };

            self.values.push(value);
            self.values.swap_remove(index);
        }

        if self.max.is_none() || value > self.max.unwrap() {
            self.max = Some(value);
        }

        if self.min.is_none() || value < self.min.unwrap() {
            self.min = Some(value);
        }
    }

    fn max(&self) -> Option<usize> {
        self.max
    }

    fn min(&self) -> Option<usize> {
        self.min
    }

    fn median(&mut self) -> Option<usize> {
        self.quantile(0.5)
    }

    fn quantile(&mut self, quantile: f32) -> Option<usize> {
        self.values.sort();

        let index = (self.values.len() as f32 * quantile - 1.0).ceil() as usize;
        Some(self.values[index])
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instantiates() {
        let stats = AggregatedStats::new();
    }

    #[test]
    fn test_max_min() {
        let mut stats = AggregatedStats::new();
        stats.add(10);
        stats.add(11);
        stats.add(9);

        assert_eq!(stats.max().unwrap(), 11);
        assert_eq!(stats.min().unwrap(), 9);
    }

    #[test]
    fn test_median() {
        let mut stats = AggregatedStats::new();
        stats.add(10);
        stats.add(11);
        stats.add(9);

        assert_eq!(stats.median().unwrap(), 10);
    }

    #[test]
    fn test_quantile() {
        let mut stats = AggregatedStats::new();
        stats.add(10);
        stats.add(11);
        stats.add(9);
        stats.add(5);

        assert_eq!(stats.quantile(0.25).unwrap(), 5);
        assert_eq!(stats.quantile(0.75).unwrap(), 10);
        assert_eq!(stats.quantile(1.0).unwrap(), 11);

    }

    #[test]
    fn test_median_approximate() {
        let mut stats = AggregatedStats::with_capacity(3);
        stats.add(2);
        stats.add(4);
        stats.add(6);

        stats.add(3);

        assert_eq!(stats.median().unwrap(), 3);
        assert_eq!(stats.values.len(), 3);
    }

    #[test]
    fn test_min_max_with_limited_capacity() {
        let mut stats = AggregatedStats::with_capacity(2);
        stats.add(10);
        stats.add(11);
        stats.add(12);

        stats.add(1);
        stats.add(100);

        assert_eq!(stats.max().unwrap(), 100);
        assert_eq!(stats.min().unwrap(), 1);
    }
}
