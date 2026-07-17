FROM ghcr.io/galoisinc/cryptol-remote-api:nightly

CMD ["--max-occupancy", "10", "http", "--host", "0.0.0.0", "--port", "8080", "/"]

EXPOSE 8080
