import numpy as np
import logging
from . import (
    _lexer,
    _parser,
    _evaluator,
    _layering,
)

logger = logging.getLogger(__name__)


def trim_missing_edges(output_image: np.ndarray) -> np.ndarray:
    return output_image


def process(img_array: np.ndarray, program: str) -> np.ndarray:
    logging.debug(f"Running program: {program!r}")

    tokens = _lexer.tokens_from_program(program)
    ast = _parser.ast_from_tokens(tokens)
    layers = _evaluator.evaluate(img_array, ast)
    output_image = _layering.combine_layers(layers)

    output_image = trim_missing_edges(output_image)

    return output_image
