FROM ghcr.io/nationalsecurityagency/foundation/cryptol-tools:latest

# Install some tools to assist with development.
RUN apt-get update -y \
 && DEBIAN_FRONTEND="noninteractive" apt-get install -y \
    vim \
    openssh-client \
    bash-completion \
    htop \
    man \
    graphviz \
    make \
    cmake \
    libgmp-dev \
    libssl-dev \
    libgomp1 \
    libnuma-dev \
    autoconf \
    build-essential \
    clang-18 \
    clang-tools-18 \
    clang-format-18 \
    emacs \
 && apt-get clean && rm -rf /var/lib/apt/lists/*

# Link clang-18 utils

RUN find /usr/bin/ -name "*-18" -exec basename {} \; | sed "s/\-18//" | xargs -I{} ln -s /usr/bin/'{}'-18 /usr/bin/'{}'

# Setup global install for cargo, rustup, and kani.
RUN mkdir /usr/local/share/.rustup
ENV RUSTUP_HOME=/usr/local/share/.rustup

RUN mkdir /usr/local/share/.cargo
ENV CARGO_HOME=/usr/local/share/.cargo

RUN mkdir /usr/local/share/.kani
ENV KANI_HOME=/usr/local/share/.kani

# Add various bins to PATH
ENV PATH="${CARGO_HOME}/bin:${RUSTUP_HOME}/bin:${KANI_HOME}/bin:${PATH}"

# Get Rust, Kani, and Bolero
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y \
 && cargo install --locked kani-verifier \
 && cargo kani setup \
 && cargo install cargo-bolero \
 && chmod a+rw -R /usr/local/share/.cargo /usr/local/share/.rustup /usr/local/share/.kani
