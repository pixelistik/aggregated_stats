struct AggregatedStats {
    values: Vec<usize>,
}

impl AggregatedStats {
    fn new() -> AggregatedStats {
        AggregatedStats { values: vec![] }
    }

    fn add(&mut self, value: usize) {
        self.values.push(value);
    }

    fn max(&self) -> Option<&usize> {
        self.values.iter().max()
    }

    fn min(&self) -> Option<&usize> {
        self.values.iter().min()
    }

    fn median(&mut self) -> Option<&usize> {
        self.quantile(0.5)
    }

    fn quantile(&mut self, quantile: f32) -> Option<&usize> {
        self.values.sort();

        let index = (self.values.len() as f32 * quantile - 1.0).ceil() as usize;
        Some(&self.values[index])
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

        assert_eq!(*stats.max().unwrap(), 11);
        assert_eq!(*stats.min().unwrap(), 9);
    }

    #[test]
    fn test_median() {
        let mut stats = AggregatedStats::new();
        stats.add(10);
        stats.add(11);
        stats.add(9);

        assert_eq!(*stats.median().unwrap(), 10);
    }

    #[test]
    fn test_quantile() {
        let mut stats = AggregatedStats::new();
        stats.add(10);
        stats.add(11);
        stats.add(9);
        stats.add(5);

        assert_eq!(*stats.quantile(0.25).unwrap(), 5);
        assert_eq!(*stats.quantile(0.75).unwrap(), 10);
    }
}
