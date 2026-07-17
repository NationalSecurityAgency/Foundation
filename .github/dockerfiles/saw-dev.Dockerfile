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

COPY --from=ghcr.io/galoisinc/crux-mir:latest --chown=root:root /home/crux-mir/.cargo/* /usr/local/bin/
COPY --from=ghcr.io/galoisinc/crux-mir:latest --chown=root:root /home/crux-mir/.rustup /usr/local/rustup
COPY --from=ghcr.io/galoisinc/crux-mir:latest /usr/local/bin/* /usr/local/bin/
COPY --from=ghcr.io/galoisinc/crux-mir:latest /crux-mir/rlibs /crux-mir/rlibs

ENV CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup \
    CRUX_RUST_LIBRARY_PATH=/crux-mir/rlibs \
    LD_LIBRARY_PATH=/usr/local/lib:/usr/local/rustup/lib
