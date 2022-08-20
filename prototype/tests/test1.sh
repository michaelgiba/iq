#!/bin/bash

set -euxo pipefail

python -m iq.main "." ./tests/data/stock-photo1.jpg

