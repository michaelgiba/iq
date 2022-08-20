#!/bin/bash
set -euxo pipefail

"$(dirname $0)/run_black.sh"
"$(dirname $0)/run_mypy.sh"
