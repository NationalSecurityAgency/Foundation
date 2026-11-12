FROM ghcr.io/nationalsecurityagency/foundation/base:latest

RUN apt-get update \
 && DEBIAN_FRONTEND="noninteractive" apt-get install --yes \
    # cryptol and saw deps
    libgmp-dev \
    libssl-dev \
    libgomp1 \
    libffi8 \
    libncurses6 \
    libtinfo6 \
    libreadline8 \
    libnuma-dev \
    openssl \
    openjdk-11-jdk-headless \
    # qbf-tools
    libglpk40 \
    libboost-iostreams1.74.0 \
    libboost-program-options1.74.0 \
    libreadline8 \
 && apt-get clean && rm -rf /var/lib/apt/lists/*

# Install SAW and saw-remote-api
COPY --from=ghcr.io/galoisinc/saw:latest /usr/local/bin /usr/local/bin
COPY --from=ghcr.io/galoisinc/saw-remote-api:latest /usr/local/bin/saw-remote-api /usr/local/bin/saw-remote-api
ENV SAW_SERVER_URL=http://0.0.0.0:36691
RUN echo 'saw-remote-api --read-only http --host 0.0.0.0 --port 36691 / &' >> /usr/local/bin/start-saw-remote-api-read-only \
 && echo 'saw-remote-api http --host 0.0.0.0 --port 36691 / &' >> /usr/local/bin/start-saw-remote-api \
 && chmod a+x /usr/local/bin/start-saw-remote-api /usr/local/bin/start-saw-remote-api-read-only

# Install Cryptol and cryptol-remote-api
COPY --from=ghcr.io/galoisinc/cryptol:latest /usr/local/bin /usr/local/bin
COPY --from=ghcr.io/galoisinc/cryptol-remote-api:latest /usr/local/bin/cryptol-remote-api /usr/local/bin/cryptol-remote-api
ENV CRYPTOL_SERVER_URL=http://0.0.0.0:36681
RUN echo 'cryptol-remote-api --read-only --max-occupancy 10 http --host 0.0.0.0 --port 36681 / &' >> /usr/local/bin/start-cryptol-remote-api-read-only \
 && echo 'cryptol-remote-api --max-occupancy 10 http --host 0.0.0.0 --port 36681 / &' >> /usr/local/bin/start-cryptol-remote-api \
 && chmod a+x /usr/local/bin/start-cryptol-remote-api /usr/local/bin/start-cryptol-remote-api-read-only

# Add --no-call-stacks option to cryptol interpreter
RUN mv /usr/local/bin/cryptol /usr/local/bin/_cryptol \
 && echo '/usr/local/bin/_cryptol --no-call-stacks $@' > /usr/local/bin/cryptol \
 && chmod a+x /usr/local/bin/cryptol

# Get what4-solvers compiled for ubuntu (pinned release with SHA-256 integrity check)
RUN wget -q https://github.com/GaloisInc/what4-solvers/releases/download/snapshot-20260622/ubuntu-24.04-X64-bin.zip \
 && echo "f4933ce5bc47d1a30d489266b86dfa6038110ab6850c96c42c34ee1327d0b705  ubuntu-24.04-X64-bin.zip" | sha256sum -c - \
 && unzip -o ubuntu-24.04-X64-bin.zip -d /usr/local/bin \
 && rm -rf ubuntu-24.04-X64-bin.zip \
 && chmod a+x /usr/local/bin/z3 /usr/local/bin/yices* /usr/local/bin/cvc* /usr/local/bin/abc /usr/local/bin/bitwuzla

# Install Python clients for Cryptol and SAW in virtual environments
ENV VENV_CRYPTOL=/opt/venv/cryptol-remote-api
ENV VENV_SAW=/opt/venv/saw-remote-api
RUN python3 -m venv $VENV_CRYPTOL \
    && $VENV_CRYPTOL/bin/pip --no-cache-dir install pyparsing tabulate numpy scipy "https://github.com/GaloisInc/cryptol/archive/refs/heads/master.tar.gz#subdirectory=cryptol-remote-api/python" \
    && python3 -m venv $VENV_SAW \
    && $VENV_SAW/bin/pip --no-cache-dir install pyparsing tabulate numpy scipy "https://github.com/GaloisInc/saw-script/archive/refs/heads/master.tar.gz#subdirectory=saw-python"
