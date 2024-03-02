ARG VARIANT=bookworm-slim
FROM espressif/idf-rust:esp32_latest
ENV DEBIAN_FRONTEND=noninteractive
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8

COPY cargo-config.toml /home/esp/.cargo/config.toml

# Set default toolchain
RUN rustup default esp

# Activate ESP environment
RUN echo "source /home/esp/export-esp.sh" >> ~/.bashrc

CMD [ "/bin/bash" ]
