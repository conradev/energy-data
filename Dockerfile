FROM debian:unstable-slim as builder

ARG TARGETPLATFORM
ARG LLVM_VERSION=15

WORKDIR /root

RUN apt-get update && \
    apt-get install -y file curl ca-certificates gnupg dirmngr make clang-$LLVM_VERSION llvm-$LLVM_VERSION musl-dev pkg-config --no-install-recommends && \
    rm -rf /var/lib/apt/lists/*

ARG ZLIB_VERSION=1.2.13

RUN case $TARGETPLATFORM in \
         "linux/arm64") LLVM_TARGET=aarch64-unknown-linux-musl ;; \
         "linux/amd64") LLVM_TARGET=x86_64-unknown-linux-musl ;; \
         *) exit 1 ;; \
    esac && \
    export CC="clang-$LLVM_VERSION -target $LLVM_TARGET" && \
    curl --proto '=https' --tlsv1.2 -sSfO https://zlib.net/zlib-$ZLIB_VERSION.tar.gz && \
    curl --proto '=https' --tlsv1.2 -sSfO https://zlib.net/zlib-$ZLIB_VERSION.tar.gz.asc && \
    tar xf zlib-$ZLIB_VERSION.tar.gz && \
    rm zlib-$ZLIB_VERSION.tar.gz zlib-$ZLIB_VERSION.tar.gz.asc && \
    cd zlib-$ZLIB_VERSION && \
    ./configure --static && \
    make && \
    make install && \
    cd .. && \
    rm -rf zlib-$ZLIB_VERSION

ENV LIBZ_SYS_STATIC=1 \
    PKG_CONFIG_PATH=/usr/local/lib/pkgconfig

ARG SQLITE_VERSION=3400100

RUN case $TARGETPLATFORM in \
         "linux/arm64") LLVM_TARGET=aarch64-unknown-linux-musl MUSL_TARGET=aarch64-linux-musl ;; \
         "linux/amd64") LLVM_TARGET=x86_64-unknown-linux-musl MUSL_TARGET=x86_64-linux-musl ;; \
         *) exit 1 ;; \
    esac && \
    curl --proto '=https' --tlsv1.2 -sSfO https://www.sqlite.org/2022/sqlite-autoconf-$SQLITE_VERSION.tar.gz && \
    tar xf sqlite-autoconf-$SQLITE_VERSION.tar.gz && \
    rm sqlite-autoconf-$SQLITE_VERSION.tar.gz && \
    cd sqlite-autoconf-$SQLITE_VERSION && \
    ./configure --disable-shared \
        CC="clang-$LLVM_VERSION -target $LLVM_TARGET" \
        CFLAGS="-I/usr/local/include -I/usr/include/$MUSL_TARGET" \
        LDFLAGS="-L/usr/local/lib -L/usr/lib/$MUSL_TARGET -L/lib/$MUSL_TARGET" && \
    make && \
    make install && \
    cd .. && \
    rm -rf sqlite-autoconf-$SQLITE_VERSION

ENV SQLITE3_STATIC=1 \
    SQLITE3_INCLUDE_DIR=/usr/local/include \
    SQLITE3_LIB_DIR=/usr/local/lib

ARG RUST_VERSION=1.68.0

RUN case $TARGETPLATFORM in \
         "linux/arm64") LLVM_TARGET=aarch64-unknown-linux-musl ;; \
         "linux/amd64") LLVM_TARGET=x86_64-unknown-linux-musl ;; \
         *) exit 1 ;; \
    esac && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
        -y \
        --profile minimal \
        --default-toolchain $RUST_VERSION \
        --target $LLVM_TARGET && \
    echo "[build]\ntarget = \"$LLVM_TARGET\"" > $HOME/.cargo/config

ENV PATH "/root/.cargo/bin:${PATH}"

WORKDIR /root/src

ENV CC_x86_64_unknown_linux_musl=clang-$LLVM_VERSION \
    AR_x86_64_unknown_linux_musl=llvm-ar-$LLVM_VERSION \
    CC_aarch64_unknown_linux_musl=clang-$LLVM_VERSION \
    AR_aarch64_unknown_linux_musl=llvm-ar-$LLVM_VERSION \
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-L/usr/lib/x86_64-linux-musl -L/lib/x86_64-linux-musl -C linker=rust-lld" \
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-L/usr/lib/aarch64-linux-musl -L/lib/aarch64-linux-musl -C linker=rust-lld" \
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUNNER="qemu-aarch64 -L /usr/aarch64-linux-gnu" \
    CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

COPY . .

RUN cargo install --path assemble

FROM scratch
COPY --from=builder /root/.cargo/bin/assemble ./
ENTRYPOINT ["/assemble"]
