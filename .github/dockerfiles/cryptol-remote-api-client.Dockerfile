FROM ghcr.io/nationalsecurityagency/foundation/base:latest

# Install Python client for Cryptol in a virtual environment
ENV VENV_CRYPTOL=/opt/venv/cryptol-remote-api
RUN python3 -m venv $VENV_CRYPTOL \
    && $VENV_CRYPTOL/bin/pip --no-cache-dir install pyparsing tabulate numpy scipy "https://github.com/GaloisInc/cryptol/archive/refs/heads/master.tar.gz#subdirectory=cryptol-remote-api/python"

RUN useradd -m cryptol && chown -R cryptol:cryptol /home/cryptol
USER cryptol
WORKDIR /home/cryptol
