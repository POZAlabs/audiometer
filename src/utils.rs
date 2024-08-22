pub fn ratio_to_db(ratio: f64, using_amplitude: bool) -> f64 {
    if ratio == 0.0 {
        return f64::INFINITY;
    }

    let logarithm = ratio.log10();
    let multiplier = if using_amplitude { 20.0 } else { 10.0 };

    multiplier * logarithm
}
