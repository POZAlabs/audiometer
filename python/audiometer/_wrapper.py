import pydub

from audiometer import _audiometer


def calculate_rms(segment: pydub.AudioSegment) -> float:
    return round(
        _audiometer.calculate_rms_inner(
            samples=segment.get_array_of_samples(),
            channels=segment.channels,
            max_amplitude=segment.max_possible_amplitude,
            sample_rate=segment.frame_rate,
        ),
        1,
    )


def calculate_peak(segment: pydub.AudioSegment) -> float:
    return round(
        _audiometer.calculate_peak_inner(
            samples=segment.get_array_of_samples(),
            channels=segment.channels,
            max_amplitude=segment.max_possible_amplitude,
        ),
        1,
    )
