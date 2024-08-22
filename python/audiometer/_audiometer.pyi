from collections.abc import MutableSequence
from pathlib import Path
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
def measure_loudness(audio_path: str | Path) -> Loudness: ...
def parse_integrated_loudness(filter_output: str) -> float: ...
def parse_momentary_loudness(filter_output: str) -> list[float]: ...
def convert_24bit_to_32bit(data: bytes) -> bytes: ...
