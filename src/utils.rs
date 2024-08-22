// Integration time 300ms을 2를 나눈 값인 150ms 만큼 지수이동평균 적용
pub const INTEGRATION_TIME: f64 = 0.3 / 2.0;
// AES17에 따라 RMS 값에 +3dB를 적용하기 위한 보정값 (log10(2) = 0.3)
pub const AMPLITUDE_COEFFICIENT: f64 = 2.0;

pub fn ratio_to_db(ratio: f64, using_amplitude: bool) -> f64 {
    if ratio == 0.0 {
        return f64::INFINITY;
    }

    let logarithm = ratio.log10();
    let multiplier = if using_amplitude { 20.0 } else { 10.0 };

    multiplier * logarithm
}
