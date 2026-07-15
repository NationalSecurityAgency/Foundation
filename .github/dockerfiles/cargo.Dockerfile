FROM ghcr.io/nationalsecurityagency/foundation/base:latest

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
