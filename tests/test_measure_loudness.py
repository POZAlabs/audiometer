from pathlib import Path

import audiometer
import pydub


def test_measure_loudness(audio_path: Path):
    expected = dict(
        integrated=-23.5,
    )

    audio_segment = pydub.AudioSegment.from_wav(audio_path)
    actual = audiometer.measure_loudness(
        samples=audio_segment.get_array_of_samples(),
        channels=audio_segment.channels,
        max_amplitude=audio_segment.max_possible_amplitude,
        sample_rate=audio_segment.frame_rate,
    )

    assert actual["integrated"] == expected["integrated"]
