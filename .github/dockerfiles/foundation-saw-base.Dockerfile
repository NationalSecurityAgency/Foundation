FROM ghcr.io/nationalsecurityagency/foundation/base:latest

RUN apt-get update \
 && DEBIAN_FRONTEND="noninteractive" apt-get install --yes \
    libgmp10 \
    libgomp1 \
    libffi8 \
    wget \
    libncurses6 \
    unzip \
    libreadline-dev \
    openjdk-11-jdk-headless \
 && apt-get clean && rm -rf /var/lib/apt/lists/*

COPY --from=ghcr.io/galoisinc/saw:latest /usr/local/bin /usr/local/bin
COPY --from=ghcr.io/galoisinc/saw-remote-api:latest /usr/local/bin/saw-remote-api /usr/local/bin/saw-remote-api

# Install Python client for SAW in a virtual environment
ENV VENV_SAW=/opt/venv/saw-remote-api
RUN python3 -m venv $VENV_SAW \
    && $VENV_SAW/bin/pip --no-cache-dir install pyparsing tabulate numpy scipy "https://github.com/GaloisInc/saw-script/archive/refs/heads/master.tar.gz#subdirectory=saw-python"

RUN useradd -m saw && chown -R saw:saw /home/saw
USER saw
WORKDIR /home/saw
