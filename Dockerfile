
FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    wget \
    curl \
    gnupg \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN wget https://bitcoincore.org/bin/bitcoin-core-25.0/bitcoin-25.0-x86_64-linux-gnu.tar.gz \
    && tar -xzf bitcoin-25.0-x86_64-linux-gnu.tar.gz \
    && install -m 0755 -o root -g root -t /usr/local/bin bitcoin-25.0/bin/* \
    && rm -rf bitcoin-25.0-x86_64-linux-gnu.tar.gz bitcoin-25.0

RUN mkdir -p /root/.bitcoin /app
WORKDIR /app

#  Bitcoin configuration
COPY scripts/bitcoin.conf /root/.bitcoin/bitcoin.conf

COPY rust-app /app/rust-app

# Building the Rust application
RUN cd /app/rust-app && cargo build --release

# Copy startup script
COPY scripts/start.sh /app/start.sh

# Enabling proper permissions and access to the script
RUN chmod +x /app/start.sh

EXPOSE 38332

#starting the application
CMD ["/app/start.sh"]
