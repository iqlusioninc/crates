# Rust CI Dockerfile (iqlusion)
#
# Resulting image is published as iqlusion/rust-ci on Docker Hub

FROM centos:7.5.1804

# Install/update RPMs
RUN yum update -y && \
    yum groupinstall -y "Development Tools" && \
    yum install -y centos-release-scl cmake epel-release libusbx-devel \
                   libudev-devel openssl-devel rpm-devel xz-devel && \
    yum install -y --enablerepo=epel libsodium-devel && \
    yum install -y --enablerepo=centos-sclo-rh llvm-toolset-7 && \
    yum clean all && \
    rm -rf /var/cache/yum

# Set environment variables to enable SCL packages (llvm-toolset-7)
ENV LD_LIBRARY_PATH=/opt/rh/llvm-toolset-7/root/usr/lib64
ENV PATH "/opt/rh/llvm-toolset-7/root/usr/bin:/opt/rh/llvm-toolset-7/root/usr/sbin:$PATH"
ENV PKG_CONFIG_PATH=/opt/rh/llvm-toolset-7/root/usr/lib64/pkgconfig
ENV X_SCLS llvm-toolset-7

# rustup configuration
ENV RUSTUP_INIT_VERSION "2018-02-13"
ENV RUSTUP_INIT "rustup-init-$RUSTUP_INIT_VERSION"
ENV RUSTUP_INIT_DIGEST "d8823472cd91d102bb469dec4d05bc8808116cd5c8ac51d87685687d6c124757"

# TODO: not root maybe?
WORKDIR /root

# Prospectively include cargo in the path
ENV PATH "$PATH:/root/.cargo/bin"

# Install rustup
RUN curl -O https://storage.googleapis.com/iqlusion-prod-artifacts/rust/$RUSTUP_INIT.xz && \
    echo "$RUSTUP_INIT_DIGEST $RUSTUP_INIT.xz" | sha256sum -c && \
    unxz $RUSTUP_INIT.xz && \
    chmod +x $RUSTUP_INIT && \
    ./$RUSTUP_INIT -y && \
    rm $RUSTUP_INIT && \
    bash -l -c "echo $(rustc --print sysroot)/lib >> /etc/ld.so.conf" && \
    ldconfig

# Install rustfmt, clippy, and additional crates
RUN rustup update && \
    rustup component add rustfmt-preview && \
    rustup component add clippy-preview && \
    cargo install cargo-audit --vers "0.5.2"

# Configure Rust environment variables
ENV RUSTFLAGS "-Ctarget-feature=+aes"
ENV RUST_BACKTRACE full
