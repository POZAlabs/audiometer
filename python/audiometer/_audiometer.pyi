from collections.abc import MutableSequence
from typing import TypedDict

class Loudness(TypedDict):
    integrated: float
    momentary: list[float]

def measure_rms(
    samples: MutableSequence[int],
    channels: int,
    max_amplitude: float,
    sample_rate: int,
) -> float: ...
def measure_peak(
    samples: MutableSequence[int],
    channels: int,
    max_amplitude: float,
) -> float: ...
def measure_loudness(
    samples: MutableSequence[int],
    channels: int,
    max_amplitude: float,
    sample_rate: int,
) -> Loudness: ...
def convert_24bit_to_32bit(data: bytes) -> bytes: ...
