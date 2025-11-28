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

## 3. Docker Distribution

The project now includes a production-ready `Dockerfile` for easy distribution.

1.  **Build the Image**:
    ```bash
    docker build -t ssebidecoin .
    ```

2.  **Run the Node**:
    ```bash
    docker run -d -p 9000:9000 --name ssebidecoin-node -v ssebidecoin-data:/data ssebidecoin
    ```

3.  **Run the Wallet**:
    ```bash
    docker run -it --rm --entrypoint wallet ssebidecoin --help
    ```

4.  **Run the Miner**:
    ```bash
    docker run -d --name ssebidecoin-miner --entrypoint miner ssebidecoin --address <YOUR_ADDRESS>
    ```

5.  **Push to Registry**:
    ```bash
    docker tag ssebidecoin yourusername/ssebidecoin:latest
    docker push yourusername/ssebidecoin:latest
    ```

## 4. Public Nodes

To bootstrap the network, you should run at least one "seed node" that is always online and publicly accessible. Hardcode this node's IP address in the `lib/src/network.rs` or configuration file so new peers can find the network.
