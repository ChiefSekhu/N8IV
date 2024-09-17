FROM gitpod/workspace-full:latest

# Install Rust for blockchain development
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
