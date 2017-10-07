pub struct AggregatedStats {
    value_buffer: Vec<usize>,
    value_count: usize,
    max_size: usize,
    max: Option<usize>,
    min: Option<usize>,
    average: Option<f32>,
}

impl AggregatedStats {
    pub fn new() -> AggregatedStats {
        Self::with_capacity(10000)
    }

    pub fn with_capacity(capacity: usize) -> AggregatedStats {
        AggregatedStats {
            value_buffer: vec![],
            value_count: 0,
            max_size: capacity,
            max: None,
            min: None,
            average: None,
        }
    }

    pub fn add(&mut self, value: usize) {
        if self.max.is_none() || value > self.max.unwrap() {
            self.max = Some(value);
        }

        if self.min.is_none() || value < self.min.unwrap() {
            self.min = Some(value);
        }

        self.average = match self.average {
            Some(current_average) => {
                Some((current_average * self.value_count as f32 + value as f32) /
                     (self.value_count as f32 + 1.0))
            }
            None => Some(value as f32),
        };

        if self.value_buffer.len() < self.max_size {
            self.value_buffer.push(value);
        } else {
            self.value_buffer.sort();
            let index = match self.value_buffer.binary_search(&value) {
                Ok(index) => index,
                Err(index) => index,
            };

            self.value_buffer.push(value);
            self.value_buffer.swap_remove(index);
        }

        self.value_count = self.value_count + 1;
    }

    pub fn max(&self) -> Option<usize> {
        self.max
    }

    pub fn min(&self) -> Option<usize> {
        self.min
    }

    pub fn median(&mut self) -> Option<f32> {
        self.quantile(0.5)
    }

    pub fn quantile(&mut self, quantile: f32) -> Option<f32> {
        if self.value_buffer.is_empty() {
            return None;
        }

        if quantile == 1.0 {
            return match self.max() {
                Some(max) => Some(max as f32),
                None => None,
            };
        }

        self.value_buffer.sort();

        let np = self.value_buffer.len() as f32 * quantile;
        let index = np.floor() as usize - 1;

        Some(match np - np.floor() {
            0.0 => (self.value_buffer[index] + self.value_buffer[index + 1]) as f32 / 2.0,
            _ => self.value_buffer[index + 1] as f32,
        })
    }

    pub fn average(&self) -> Option<f32> {
        self.average
    }

    pub fn count(&self) -> usize {
        self.value_count
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_approximate() {
        let mut stats = AggregatedStats::with_capacity(3);
        stats.add(2);
        stats.add(4);
        stats.add(6);

        stats.add(3);

        assert_eq!(stats.median().unwrap(), 3.0);
        assert_eq!(stats.value_buffer.len(), 3);
    }
}
