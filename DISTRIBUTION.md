# Distributing Ssebidecoin

This guide outlines the steps to make Ssebidecoin available for public use.

## 1. Building Binaries

To allow users to run Ssebidecoin without installing Rust, you should provide pre-compiled binaries for major operating systems (Windows, Linux, macOS).

### Windows
```powershell
cargo build --release --target x86_64-pc-windows-msvc
# Binaries will be in target/release/
# - node.exe
# - wallet.exe
# - miner.exe
```

### Linux
```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

### macOS
```bash
cargo build --release --target x86_64-apple-darwin
```

## 2. Creating a Release

1.  **Tag the Version**:
    ```bash
    git tag -a v0.1.0 -m "Initial Release"
    git push origin v0.1.0
    ```

2.  **GitHub Releases**:
    -   Go to your GitHub repository -> **Releases** -> **Draft a new release**.
    -   Select the tag `v0.1.0`.
    -   Upload the compiled binaries (zip them for easier downloading, e.g., `ssebidecoin-v0.1.0-windows.zip`).
    -   Publish the release.

## 3. Docker (Optional but Recommended)

Containerization makes it easy for users to run a node without worrying about dependencies.

1.  **Create a `Dockerfile`**:
    ```dockerfile
    FROM rust:latest as builder
    WORKDIR /usr/src/ssebidecoin
    COPY . .
    RUN cargo install --path node

    FROM debian:buster-slim
    COPY --from=builder /usr/local/cargo/bin/node /usr/local/bin/node
    CMD ["node"]
    ```

2.  **Build and Push**:
    ```bash
    docker build -t yourusername/ssebidecoin-node:latest .
    docker push yourusername/ssebidecoin-node:latest
    ```

3.  **Usage**:
    ```bash
    docker run -d --name ssebidecoin-node yourusername/ssebidecoin-node:latest
    ```

## 4. Public Nodes

To bootstrap the network, you should run at least one "seed node" that is always online and publicly accessible. Hardcode this node's IP address in the `lib/src/network.rs` or configuration file so new peers can find the network.
