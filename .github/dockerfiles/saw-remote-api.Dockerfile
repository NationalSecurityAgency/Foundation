FROM ghcr.io/galoisinc/saw-remote-api:nightly

CMD ["http", "--host", "0.0.0.0", "--port", "8080", "/"]

EXPOSE 8080
