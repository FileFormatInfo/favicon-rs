#!/bin/bash

set -o errexit
set -o pipefail
set -o nounset

docker build \
	--build-arg COMMIT=$(git rev-parse --short HEAD) \
	--build-arg LASTMOD=$(date -u +%Y-%m-%dT%H:%M:%SZ) \
	--progress plain \
	--tag favicon-rs \
	.

echo "INFO: running"
docker run \
	--env PORT='4000' \
	--expose 4000 \
	--interactive \
	--name "favicon-rs" \
	--publish 4000:4000 \
	--rm \
	--tty \
	favicon-rs

