###############################################
#                 Build stage                 #
###############################################
FROM rust:1.83 AS builder
ARG TARGETPLATFORM
ARG BUILDPLATFORM

COPY . /src
WORKDIR /src

RUN <<EOF
  export RUSTFLAGS='-C target-feature=+crt-static'
  case "$TARGETPLATFORM" in
    *"linux/amd64"*)
      echo "building for linux/amd64"
      cargo build --release --target x86_64-unknown-linux-gnu
      mv ./target/x86_64-unknown-linux-gnu/release/rizzybox /bin/rizzybox;;
    *"linux/arm64"*)
      echo "building for linux/arm64"
      cargo build --release --target aarch64-unknown-linux-gnu
      mv ./target/aarch64-unknown-linux-gnu/release/rizzybox /bin/rizzybox;;
    # *"linux/riscv64"*) # not supported by base image
    #   echo "building for linux/riscv64"
    #   cargo build --release --target riscv64gc-unknown-linux-gnu
    #   mv ./target/riscv64gc-unknown-linux-gnu/release/rizzybox /bin/rizzybox;;
    *)
      echo "unsupported target platform: $TARGETPLATFORM"
      exit 1;;
  esac
EOF

RUN <<EOF
  for bin in $(rizzybox --list); do
    echo "ln -sf /bin/rizzybox /usr/local/bin/$bin"
    ln -sf /bin/rizzybox /usr/local/bin/$bin
  done
EOF

RUN ls /usr/local/bin/sh || exit 1

###############################################
#                  App stage                  #
###############################################
FROM scratch
COPY --from=builder /bin/rizzybox /bin/rizzybox
COPY --from=builder /usr/local/bin /usr/local/bin
ENTRYPOINT ["/bin/rizzybox"]
CMD ["sh"]
