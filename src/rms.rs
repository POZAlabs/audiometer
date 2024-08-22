use pyo3::pyfunction;

use crate::utils::{AMPLITUDE_COEFFICIENT, INTEGRATION_TIME, ratio_to_db};

#[pyfunction]
pub fn measure_rms(
    samples: Vec<isize>,
    channels: usize,
    max_amplitude: f64,
    sample_rate: isize,
) -> f64 {
    let decay_const = (-1.0 / sample_rate as f64 / INTEGRATION_TIME).exp();
    let update_ratio = 1.0 - decay_const;

    let mut max_rms: f64 = 0.0;
    for i in 0..channels {
        let mut channel_max_rms: f64 = 0.0;
        let mut current_rms: f64 = 0.0;
        for channel_sample in samples[i..].iter().step_by(channels) {
            let sample = (*channel_sample as f64 / max_amplitude).abs();
            current_rms = (current_rms * decay_const) + (sample * sample * update_ratio);
            channel_max_rms = channel_max_rms.max(current_rms);
        }

        max_rms = max_rms.max(channel_max_rms);
    }

    ratio_to_db(max_rms * AMPLITUDE_COEFFICIENT, false)
}
