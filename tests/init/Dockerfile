# The image for testing the initialize scripts of sibling.
# It has every shell which "sibling --init" supports.
#
# Build it by "docker build -t sibling-init-test tests/init", or simply run
# tests/init/run.sh, which builds it for you.

FROM debian:bookworm-slim

# TARGETARCH is given by BuildKit; amd64 or arm64.
ARG TARGETARCH
ARG ELVISH_VERSION=0.21.0
ARG POWERSHELL_VERSION=7.6.4

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        bash zsh fish sed coreutils ca-certificates curl libicu72 libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Elvish and PowerShell are not in the Debian archive; get their releases.
RUN set -eu; \
    case "${TARGETARCH}" in \
        amd64) elvish_arch=amd64; pwsh_arch=x64 ;; \
        arm64) elvish_arch=arm64; pwsh_arch=arm64 ;; \
        *) echo "${TARGETARCH}: unsupported architecture" >&2; exit 1 ;; \
    esac; \
    curl -sSL "https://dl.elv.sh/linux-${elvish_arch}/elvish-v${ELVISH_VERSION}.tar.gz" \
        | tar zx -C /usr/local/bin elvish; \
    mkdir -p /opt/powershell; \
    curl -sSL "https://github.com/PowerShell/PowerShell/releases/download/v${POWERSHELL_VERSION}/powershell-${POWERSHELL_VERSION}-linux-${pwsh_arch}.tar.gz" \
        | tar zx -C /opt/powershell; \
    chmod +x /opt/powershell/pwsh; \
    ln -s /opt/powershell/pwsh /usr/local/bin/pwsh

# The stand-ins for the filter commands, which let the tests call the public
# sibling_peco and sibling_fzf functions: "peco" always chooses the 5th line of
# the list, and "fzf" chooses nothing, as the user who quit it.
RUN printf '#!/bin/sh\nsed -n "5p"\n' > /usr/local/bin/peco \
    && chmod +x /usr/local/bin/peco \
    && ln -s /bin/true /usr/local/bin/fzf

# The tests run as the host user, who has no entry in /etc/passwd.
ENV HOME=/tmp
WORKDIR /work
