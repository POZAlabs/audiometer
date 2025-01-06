import math
from pathlib import Path

import audiometer
import pydub


def test_measure_loudness(audio_path: Path):
    expected = dict(
        integrated=-23.5,
        momentary=[
            -math.inf,
            -math.inf,
            -math.inf,
            -44.6,
            -40.1,
            -36.2,
            -33.4,
            -31.1,
            -28.9,
            -26.6,
            -24.2,
            -21.4,
            -18.1,
        ],
    )

    audio_segment = pydub.AudioSegment.from_wav(audio_path)
    actual = audiometer.measure_loudness(
        samples=audio_segment.get_array_of_samples(),
        channels=audio_segment.channels,
        max_amplitude=audio_segment.max_possible_amplitude,
        sample_rate=audio_segment.frame_rate,
    )

    assert actual == expected
