use pyo3::prelude::*;

mod lufs;
mod peak;
mod rms;
mod sample;
mod types;
mod utils;

#[pymodule]
fn _audiometer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rms::measure_rms, m)?)?;
    m.add_function(wrap_pyfunction!(peak::measure_peak, m)?)?;
    m.add_function(wrap_pyfunction!(lufs::measure_loudness, m)?)?;
    m.add_function(wrap_pyfunction!(sample::convert_24bit_to_32bit, m)?)?;
    Ok(())
}
