FROM ghcr.io/nationalsecurityagency/foundation/base:latest

COPY --from=ghcr.io/galoisinc/crux-mir:latest --chown=root:root /home/crux-mir/.cargo/* /usr/local/bin/
COPY --from=ghcr.io/galoisinc/crux-mir:latest --chown=root:root /home/crux-mir/.rustup /usr/local/rustup
COPY --from=ghcr.io/galoisinc/crux-mir:latest /usr/local/bin/* /usr/local/bin/
COPY --from=ghcr.io/galoisinc/crux-mir:latest /crux-mir/rlibs /crux-mir/rlibs

ENV CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup \
    CRUX_RUST_LIBRARY_PATH=/crux-mir/rlibs \
    LD_LIBRARY_PATH=/usr/local/lib:/usr/local/rustup/lib
