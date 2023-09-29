use pyo3::prelude::*;
use regex::Regex;

#[pyfunction]
pub fn parse_integrated_loudness(filter_output: &str) -> f64 {
    let output_pattern = Regex::new(
        r"(?P<label>Integrated loudness:)(?P<whitespace>\n\s+)(?P<value>I:\s+?[-]?\d+\.\d+\s+?LUFS)"
    ).unwrap();
    let value_pattern = Regex::new(r"-?\d+\.\d+").unwrap();

    let outputs = output_pattern
        .captures_iter(filter_output)
        .filter_map(|cap| {
            let value = cap.name("value");
            match value {
                Some(value) => Some(value.as_str()),
                None => None,
            }
        })
        .filter_map(|value| {
            let value = value_pattern.find(value);
            match value {
                Some(value) => Some(value.as_str()),
                None => None,
            }
        })
        .filter_map(|value| {
            let parsed_value = value.parse::<f64>().ok();
            if parsed_value == Some(-70.0) {
                None
            } else {
                parsed_value
            }
        })
        .collect::<Vec<f64>>();

    *outputs.first().unwrap()
}
