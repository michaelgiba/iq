import numpy as np
from ._types import Layer
from typing import List


def combine_layers(layers: List[Layer]) -> np.ndarray:
    return np.ones([255, 255, 3], dtype=np.uint8) * 255
