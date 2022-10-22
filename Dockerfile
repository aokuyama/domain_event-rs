FROM rust:1.63.0-slim

RUN apt-get update && \
    apt-get install -y git libssl-dev pkg-config curl
# install mold
RUN curl -LO https://github.com/rui314/mold/releases/download/v1.4.2/mold-1.4.2-aarch64-linux.tar.gz &&\
    tar zxf mold-1.4.2-aarch64-linux.tar.gz -C /opt &&\
    ln -s /opt/mold-1.4.2-aarch64-linux/bin/mold /bin/mold &&\
    rm mold-1.4.2-aarch64-linux.tar.gz

RUN rustup component add rustfmt
