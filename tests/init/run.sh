#!/bin/bash
#
# Test the initialize scripts which "sibling --init" generates, by running the
# utility commands on every supported shell in a container.
#
# The shells must behave identically, hence, the stdout and the stderr of each
# of them are compared with expected.out and expected.err in this directory.
# Note that the two streams are compared separately, since the order of writing
# to them differs among the shells.
#
# Usage:
#     tests/init/run.sh [SHELL...]   test the given shells (default: all of them)
#     tests/init/run.sh --update     update the expected files by the bash result
#
# Requirements: docker. The image is built from the Dockerfile in this
# directory, and the command is built by the rust image; both of them are
# cached, hence, only the first run takes a while.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "${here}/../.." && pwd)"
work="${root}/target/init-test"
image="${SIBLING_TEST_IMAGE:-sibling-init-test}"
rust_image="${SIBLING_RUST_IMAGE:-rust:1-slim}"

all_shells=(bash zsh fish elvish powershell)
update=no

usage() {
    sed -n '2,17p' "$0" | sed 's/^#[[:space:]]\{0,1\}//'
}

case "${1:-}" in
    --update) update=yes; shift ;;
    -h | --help) usage; exit 0 ;;
esac
if [ $# -gt 0 ]; then
    shells=("$@")
else
    shells=("${all_shells[@]}")
fi

if ! docker info > /dev/null 2>&1; then
    echo "docker is not available; this test runs the shells in a container." >&2
    exit 1
fi

mkdir -p "${work}"

echo "==> building the image ${image}"
docker build --quiet --tag "${image}" "${here}" > /dev/null

echo "==> building the sibling command for the container"
docker run --rm \
    --user "$(id -u):$(id -g)" \
    --volume "${root}:/work" \
    --workdir /work \
    --env CARGO_HOME=/work/target/init-test/cargo-home \
    "${rust_image}" \
    cargo build --release --quiet --target-dir /work/target/init-test/cargo

# The command line of each shell; every one of them runs the same cases.
shell_command() {
    # shellcheck disable=SC2016 # the variables of the elvish line are expanded in the container
    case "$1" in
        bash) echo "bash --norc /work/tests/init/cases.sh" ;;
        zsh) echo "zsh -f /work/tests/init/cases.sh" ;;
        fish) echo "fish --no-config /work/tests/init/cases.fish" ;;
        powershell) echo "pwsh -NoProfile -File /work/tests/init/cases.ps1" ;;
        # Elvish loads the script as a module; install it as the document says.
        elvish) echo 'sh -c "mkdir -p \"${XDG_CONFIG_HOME}/elvish/lib\" && sibling --init elvish > \"${XDG_CONFIG_HOME}/elvish/lib/sibling.elv\" && exec elvish /work/tests/init/cases.elv"' ;;
        *) echo "$1: unknown shell" >&2; return 1 ;;
    esac
}

run_shell() {
    local name="$1"
    local command
    command="$(shell_command "${name}")"
    # The exit status of the last case is not 0; the cases are told by the
    # output, not by the status of the whole run.
    docker run --rm \
        --user "$(id -u):$(id -g)" \
        --volume "${root}:/work" \
        --workdir /work \
        --env PATH=/work/target/init-test/cargo/release:/usr/local/bin:/usr/bin:/bin \
        --env RUST_LOG=off \
        --env HOME=/tmp \
        --env XDG_CONFIG_HOME=/tmp/config \
        --env "TEST_SHELL=${name}" \
        "${image}" \
        sh -c "${command}" \
        > "${work}/${name}.out" 2> "${work}/${name}.err" || true
}

if [ "${update}" = yes ]; then
    echo "==> running bash to update the expected files"
    run_shell bash
    cp "${work}/bash.out" "${here}/expected.out"
    cp "${work}/bash.err" "${here}/expected.err"
    echo "updated: ${here}/expected.out, ${here}/expected.err"
    echo "review them, and run this script without --update to test every shell."
    exit 0
fi

failed=0
for name in "${shells[@]}"; do
    printf '==> %-11s' "${name}"
    run_shell "${name}"
    ok=yes
    for stream in out err; do
        if ! diff -u "${here}/expected.${stream}" "${work}/${name}.${stream}" \
            > "${work}/${name}.${stream}.diff" 2>&1; then
            ok=no
        fi
    done
    if [ "${ok}" = yes ]; then
        echo "ok"
    else
        echo "FAILED"
        failed=$((failed + 1))
        for stream in out err; do
            if [ -s "${work}/${name}.${stream}.diff" ]; then
                echo "--- ${name}: the difference of the std${stream} ---"
                sed 's/^/    /' "${work}/${name}.${stream}.diff"
            fi
        done
    fi
done

echo
if [ "${failed}" -eq 0 ]; then
    echo "all of ${#shells[@]} shells behave as expected."
else
    echo "${failed} of ${#shells[@]} shells failed; the outputs are in ${work}."
    exit 1
fi
