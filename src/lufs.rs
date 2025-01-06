use crate::types;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyfunction]
pub fn measure_loudness(
    py: Python<'_>,
    samples: types::Samples,
    channels: usize,
    max_amplitude: f64,
    sample_rate: usize,
) -> Result<&PyDict, PyErr> {
    let mut meter = ebur128::EbuR128::new(
        channels as u32,
        sample_rate as u32,
        ebur128::Mode::I | ebur128::Mode::M,
    )
    .map_err(|e| PyValueError::new_err(e.to_string()))?;

    let samples_in_100ms = (sample_rate + 5) / 10;
    let chunk_size = channels * samples_in_100ms;
    let mut momentary = Vec::new();
    for chunk in samples.normalized_source(max_amplitude).chunks(chunk_size) {
        meter
            .add_frames_f64(chunk)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        if let Ok(m) = meter.loudness_momentary() {
            momentary.push(round_loudness(m));
        }
    }
    let integrated = round_loudness(meter.loudness_global().unwrap_or(f64::NEG_INFINITY));

    let result = PyDict::new(py);
    result.set_item("integrated", integrated)?;
    result.set_item("momentary", momentary)?;

    Ok(result)
}

fn round_loudness(f: f64) -> f64 {
    (f * 10.0).round() / 10.0
}
