from pathlib import Path

import pytest


@pytest.fixture
def audio_path() -> Path:
    return Path(__file__).parent.joinpath("test.wav")
