FROM ghcr.io/galoisinc/saw-remote-api:latest

CMD ["http", "--host", "0.0.0.0", "--port", "8080", "/"]

EXPOSE 8080
