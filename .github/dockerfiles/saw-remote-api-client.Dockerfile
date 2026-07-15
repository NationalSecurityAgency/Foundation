FROM ghcr.io/nationalsecurityagency/foundation/base:latest

# Install Python client for SAW in a virtual environment
ENV VENV_SAW=/opt/venv/saw-remote-api
RUN python3 -m venv $VENV_SAW \
    && $VENV_SAW/bin/pip --no-cache-dir install pyparsing tabulate numpy scipy "https://github.com/GaloisInc/saw-script/archive/refs/heads/master.tar.gz#subdirectory=saw-python"

RUN useradd -m saw && chown -R saw:saw /home/saw
USER saw
WORKDIR /home/saw
