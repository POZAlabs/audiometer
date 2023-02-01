from collections.abc import MutableSequence

def calculate_rms_inner(
    samples: MutableSequence[int],
    channels: int,
    max_amplitude: float,
    sample_rate: int,
) -> float: ...
def calculate_peak_inner(
    samples: MutableSequence[int],
    channels: int,
    max_amplitude: float,
) -> float: ...
