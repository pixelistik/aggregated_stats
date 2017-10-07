extern crate aggregated_stats;

use aggregated_stats::*;

#[test]
fn test_instantiates() {
    let _ = AggregatedStats::new();
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

    assert_eq!(stats.median().unwrap(), 10.0);
}

#[test]
fn test_median_even_count() {
    let mut stats = AggregatedStats::new();
    stats.add(1);
    stats.add(2);
    stats.add(3);
    stats.add(4);

    assert_eq!(stats.median().unwrap(), 2.5);
}

#[test]
fn test_quantile() {
    let mut stats = AggregatedStats::new();

    assert!(stats.quantile(0.5).is_none());

    stats.add(10);
    stats.add(11);
    stats.add(9);
    stats.add(5);

    assert_eq!(stats.quantile(0.25).unwrap(), 7.0);
    assert_eq!(stats.quantile(0.75).unwrap(), 10.5);
    assert_eq!(stats.quantile(1.0).unwrap(), 11.0);

}

#[test]
fn test_median_approximate() {
    let mut stats = AggregatedStats::with_capacity(3);
    stats.add(2);
    stats.add(4);
    stats.add(6);

    stats.add(3);

    assert_eq!(stats.median().unwrap(), 3.0);
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

#[test]
fn test_average() {
    let mut stats = AggregatedStats::with_capacity(1);
    stats.add(10);
    assert_eq!(stats.average().unwrap(), 10.0);

    stats.add(0);
    assert_eq!(stats.average().unwrap(), 5.0);

    stats.add(110);
    assert_eq!(stats.average().unwrap(), 40.0);
}

#[test]
fn test_count() {
    let mut stats = AggregatedStats::new();
    assert_eq!(stats.count(), 0);

    stats.add(10);
    assert_eq!(stats.count(), 1);
}

#[test]
fn test_count_more_than_capacity() {
    let mut stats = AggregatedStats::with_capacity(1);

    stats.add(10);
    stats.add(11);
    assert_eq!(stats.count(), 2);
}

#[test]
fn test_no_overflow() {
    let mut stats = AggregatedStats::new();

    stats.add(10);
    assert_eq!(stats.median().unwrap(), 10.0);
}
