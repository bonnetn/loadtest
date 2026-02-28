//! Cumulative distribution function (CDF) for latency percentiles.
//!
//! Computes percentile points over a sorted list of latencies using a
//! log-spaced tail (e.g. p50, p90, p99, p99.9).

use std::f64::consts::LN_10;
use std::time::Duration;

/// Compute CDF percentiles for a sorted list of latencies.
///
/// Parameters:
/// - `start`: initial tail value (e.g. 1.0 for 100th percentile).
/// - `resolution`: number of steps per decade (e.g. 16).
/// - `steps`: number of decades to cover (e.g. 4 → 1.0 down to 0.0001).
/// - `durations`: latencies; will be sorted in place.
///
/// Returns `(percentile, duration)` pairs with percentile in [0, 1].
pub fn calculate_cdf<'a>(
    start: f64,
    resolution: u64,
    steps: u64,
    durations: impl Iterator<Item = &'a Duration>,
) -> Vec<(f64, Duration)> {
    let mut durations = durations.copied().collect::<Vec<_>>();
    durations.sort();

    let resolution_f64 =
        f64::from(u32::try_from(resolution).expect("resolution fits in u32 for CDF"));
    let k = -LN_10 / resolution_f64;
    let ratio = k.exp();
    let mut r = start;

    let n = steps
        .checked_mul(resolution)
        .expect("CDF step count overflow");
    let n_usize = usize::try_from(n).expect("CDF step count fits in usize");
    let mut result = Vec::with_capacity(n_usize);

    let len = durations.len();
    for _ in 0..=n {
        let p = 1.0_f64 - r;
        let p_scaled = (p * 1e9).round().clamp(0.0, 1e9);
        let p_scaled = format!("{p_scaled:.0}")
            .parse::<u64>()
            .expect("p_scaled in 0..=1e9 parses as u64");
        let idx = u64::try_from(len)
            .expect("len fits in u64")
            .checked_mul(p_scaled)
            .and_then(|v| v.checked_div(1_000_000_000))
            .expect("CDF index overflow");
        let idx = usize::try_from(idx).expect("CDF index fits in usize");
        if let Some(value) = durations.get(idx) {
            result.push((p, *value));
        }
        r *= ratio;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ns(n: u64) -> Duration {
        Duration::from_nanos(n)
    }

    /// Empty input produces empty CDF.
    #[test]
    fn empty_input() {
        let input: Vec<Duration> = vec![];
        let got = calculate_cdf(1.0, 16, 4, input.iter());
        let expected: Vec<(f64, Duration)> = vec![];
        assert_eq!(got, expected);
    }

    /// Single latency: all percentile points map to that value.
    #[test]
    fn single_latency() {
        let input = vec![ns(100)];
        let got = calculate_cdf(1.0, 16, 4, input.iter());
        assert!(!got.is_empty());
        assert!(
            got.iter()
                .all(|(p, d)| (*p >= 0.0 && *p <= 1.0) && *d == ns(100))
        );
    }

    /// Two latencies: exact hardcoded output for start=1, resolution=2, steps=2.
    #[test]
    fn two_latencies_hardcoded() {
        let input = vec![ns(10), ns(20)];
        let got = calculate_cdf(1.0, 2, 2, input.iter());
        // n=4, 5 steps. ratio=10^(-1/2). p = 1 - r with r = 1, ratio, ratio^2, ratio^3, ratio^4.
        let expected = vec![
            (0.0, ns(10)),
            (0.683_772_233_983_162, ns(20)),
            (0.9, ns(20)),
            (0.968_377_223_398_316_2, ns(20)),
            (0.99, ns(20)),
        ];
        assert_eq!(got.len(), expected.len(), "length mismatch");
        for (i, ((p_got, d_got), (p_exp, d_exp))) in got.iter().zip(expected.iter()).enumerate() {
            assert!(
                (*p_got - p_exp).abs() < 1e-9,
                "index {}: percentile {} vs {}",
                i,
                p_got,
                p_exp
            );
            assert_eq!(d_got, d_exp, "index {}: duration mismatch", i);
        }
    }

    /// Three latencies: exact hardcoded output for start=1, resolution=2, steps=2.
    #[test]
    fn three_latencies_hardcoded() {
        let input = vec![ns(100), ns(200), ns(300)];
        let got = calculate_cdf(1.0, 2, 2, input.iter());
        let expected = vec![
            (0.0, ns(100)),
            (0.683_772_233_983_162, ns(300)),
            (0.9, ns(300)),
            (0.968_377_223_398_316_2, ns(300)),
            (0.99, ns(300)),
        ];
        assert_eq!(got.len(), expected.len(), "length mismatch");
        for (i, ((p_got, d_got), (p_exp, d_exp))) in got.iter().zip(expected.iter()).enumerate() {
            assert!(
                (*p_got - p_exp).abs() < 1e-9,
                "index {}: percentile {} vs {}",
                i,
                p_got,
                p_exp
            );
            assert_eq!(d_got, d_exp, "index {}: duration mismatch", i);
        }
    }

    /// Unsorted input is sorted before computing percentiles.
    #[test]
    fn unsorted_input_sorted_by_cdf() {
        let input = vec![ns(300), ns(100), ns(200)];
        let got = calculate_cdf(1.0, 2, 2, input.iter());
        let sorted_latencies: Vec<u64> = got.iter().map(|(_, d)| d.as_nanos() as u64).collect();
        let mut sorted = sorted_latencies.clone();
        sorted.sort();
        assert_eq!(sorted_latencies, sorted);
    }

    /// CDF percentiles are non-decreasing in latency (each step uses higher or equal index).
    #[test]
    fn output_latencies_non_decreasing() {
        let input: Vec<Duration> = (0..100).map(ns).collect();
        let got = calculate_cdf(1.0, 4, 2, input.iter());
        let latencies: Vec<u64> = got.iter().map(|(_, d)| d.as_nanos() as u64).collect();
        for w in latencies.windows(2) {
            assert!(
                w[0] <= w[1],
                "latencies should be non-decreasing: {:?}",
                latencies
            );
        }
    }

    /// Percentiles are in [0, 1] and increase (except possible duplicates from index rounding).
    #[test]
    fn percentiles_in_valid_range() {
        let input: Vec<Duration> = (0..50).map(|i| ns(i * 10)).collect();
        let got = calculate_cdf(1.0, 8, 3, input.iter());
        let mut prev_p = -1.0_f64;
        for (p, _) in &got {
            assert!(*p >= 0.0 && *p <= 1.0, "percentile {} out of range", p);
            assert!(
                *p >= prev_p,
                "percentiles should be non-decreasing: {} then {}",
                prev_p,
                p
            );
            prev_p = *p;
        }
    }
}
