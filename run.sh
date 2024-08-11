#!/usr/bin/env bash
#
# run locally for dev
#

set -o errexit
set -o pipefail
set -o nounset

ENV_FILE="${1:-./.env}"
if [ -f "${ENV_FILE}" ]; then
    echo "INFO: loading '${ENV_FILE}'!"
    export $(cat "${ENV_FILE}")
fi

export LASTMOD=$(date -u +%Y-%m-%dT%H:%M:%SZ)
if [[ $(git status --short) != '' ]]; then
  export COMMIT="$(git rev-parse --short HEAD) (dirty)"
else
  export COMMIT="$(git rev-parse --short HEAD)"
fi


# run the app
cargo watch --clear --exec run