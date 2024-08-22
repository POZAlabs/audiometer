use std::path::PathBuf;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use regex::Regex;

#[pyfunction]
pub fn measure_loudness<'py>(py: Python<'py>, audio_path: PathBuf) -> &PyDict {
    let filter_output = apply_ebu128_filter(audio_path.to_str().unwrap());

    let result = PyDict::new(py);
    result
        .set_item("integrated", parse_integrated_loudness(&filter_output))
        .unwrap();
    result
        .set_item("momentary", parse_momentary_loudness(&filter_output))
        .unwrap();

    result
}

fn apply_ebu128_filter(input_path: &str) -> String {
    let output = std::process::Command::new("ffmpeg")
        .args([
            "-i",
            input_path,
            "-filter_complex",
            "ebur128=peak=true",
            "-f",
            "null",
            "-",
        ])
        .output()
        .expect("Failed to execute command using ffmpeg");

    std::str::from_utf8(&output.stderr).unwrap().to_string()
}

#[pyfunction]
pub fn parse_integrated_loudness(filter_output: &str) -> f64 {
    let output_pattern = Regex::new(
        r"(?P<label>Integrated loudness:)(?P<whitespace>\n\s+)(?P<value>I:\s+?-?\d+\.\d+\s+?LUFS)",
    )
    .unwrap();
    let value_pattern = Regex::new(r"-?\d+\.\d+").unwrap();

    let outputs = output_pattern
        .captures_iter(filter_output)
        .filter_map(|cap| {
            let matched = cap.name("value");
            match matched {
                Some(value) => Some(value.as_str()),
                None => None,
            }
        })
        .filter_map(|value| {
            let matched = value_pattern.find(value);
            match matched {
                Some(value) => Some(value.as_str()),
                None => None,
            }
        })
        .filter_map(|value| {
            if value == "-70.0" {
                None
            } else {
                value.parse::<f64>().ok()
            }
        })
        .collect::<Vec<f64>>();

    *outputs.first().unwrap_or(&f64::INFINITY)
}

#[pyfunction]
pub fn parse_momentary_loudness(filter_output: &str) -> Vec<f64> {
    let output_pattern = Regex::new(r"(\[Parsed.+] t.*)").unwrap();
    let value_pattern = Regex::new(r"(?P<label>M:(\s+)?)(?P<value>-?\d+\.\d+|\w+)").unwrap();

    return output_pattern
        .captures_iter(filter_output)
        .filter_map(|cap| {
            let matched = cap.get(0);
            match matched {
                Some(matched) => Some(matched.as_str()),
                None => None,
            }
        })
        .filter_map(|value| {
            let cap = value_pattern.captures(value);
            match cap {
                Some(matched) => Some(matched.name("value")?.as_str()),
                None => None,
            }
        })
        .filter_map(|value| {
            let value = value.parse::<f64>().ok().unwrap();
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        })
        .collect::<Vec<f64>>();
}
