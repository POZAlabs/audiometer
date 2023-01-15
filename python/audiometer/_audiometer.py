import pydub

from .audiometer import calculate_rms_inner


def calculate_rms(segment: pydub.AudioSegment) -> float:
    return round(
        calculate_rms_inner(
            samples=segment.get_array_of_samples(),
            channels=segment.channels,
            max_amplitude=segment.max_possible_amplitude,
            sample_rate=segment.frame_rate,
        ),
        1,
    )
