# Base image, Rust + cargo
FROM rust
RUN ["cargo", "install", "cargo-make"]
CMD ["/bin/sh", "-c", "/bin/sh"]
