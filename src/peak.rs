use pyo3::pyfunction;

use crate::utils::ratio_to_db;

#[pyfunction]
pub fn measure_peak(samples: Vec<isize>, channels: usize, max_amplitude: f64) -> f64 {
    let mut max_peak: f64 = 0.0;
    for i in 0..channels {
        let mut channel_max_peak: f64 = 0.0;
        for channel_sample in samples[i..].iter().step_by(channels) {
            let sample = (*channel_sample as f64 / max_amplitude).abs();
            channel_max_peak = channel_max_peak.max(sample)
        }

        max_peak = max_peak.max(channel_max_peak)
    }

    ratio_to_db(max_peak, true)
}
