import array
import math

import audiometer


def test_measure_rms_return_negative_infinity_for_silence():
    samples = array.array("i", [0] * 4800)
    result = audiometer.measure_rms(
        samples=samples, channels=1, max_amplitude=2147483648.0, sample_rate=48000
    )
    assert result == -math.inf


def test_measure_peak_return_negative_infinity_for_silence():
    samples = array.array("i", [0] * 4800)
    result = audiometer.measure_peak(samples=samples, channels=1, max_amplitude=2147483648.0)
    assert result == -math.inf
