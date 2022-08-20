import sys
import logging
import numpy as np
from PIL import Image
from . import _iq

logger = logging.getLogger(__name__)


def main():
    logging.basicConfig(
        format="%(asctime)s:%(module)r - %(levelname)s - %(message)s",
        level=logging.DEBUG,
    )

    assert len(sys.argv) == 3, "Usage: iq <program> <file>"
    _, program, image_file_path = sys.argv

    logger.debug(f"Running program: {program!r}")

    image = Image.open(image_file_path)
    img_array = np.array(image)

    output_img_array = _iq.process(img_array, program)

    image = Image.fromarray(output_img_array)
    image.save("out.jpg")


if __name__ == "__main__":
    main()
