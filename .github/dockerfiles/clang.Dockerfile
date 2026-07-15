FROM ghcr.io/nationalsecurityagency/foundation/base:latest

RUN apt-get update -y \
 && DEBIAN_FRONTEND="noninteractive" apt-get install -y \
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
 && apt-get clean && rm -rf /var/lib/apt/lists/*

# Link clang-18 utils

RUN find /usr/bin/ -name "*-18" -exec basename {} \; | sed "s/\-18//" | xargs -I{} ln -s /usr/bin/'{}'-18 /usr/bin/'{}'
