###############################################
#                 Build stage                 #
###############################################
FROM messense/cargo-zigbuild AS builder
ARG TARGETPLATFORM
ARG BUILDPLATFORM

COPY . /src
WORKDIR /src

RUN <<EOF
  case "$TARGETPLATFORM" in
    *"linux/amd64"*)
      echo "building for linux/amd64"
      cargo zigbuild --release --target x86_64-unknown-linux-musl
      mv ./target/x86_64-unknown-linux-musl/release/rizzybox /bin/rizzybox;;
    *"linux/arm64"*)
      echo "building for linux/arm64"
      cargo zigbuild --release --target aarch64-unknown-linux-musl
      mv ./target/aarch64-unknown-linux-musl/release/rizzybox /bin/rizzybox;;
    # *"linux/riscv64"*) # not supported by the cargo-zigbuild image
    #   echo "building for linux/arm64"
    #   cargo zigbuild --release --target riscv64gc-unknown-linux-musl
    #   mv ./target/riscv64gc-unknown-linux-musl/release/rizzybox /bin/rizzybox;;
    *)
      echo "unsupported target platform: $TARGETPLATFORM"
      exit 1;;
  esac
EOF

###############################################
#                  App stage                  #
###############################################
FROM scratch
COPY --from=builder /bin/rizzybox /bin/rizzybox
ENTRYPOINT ["/bin/rizzybox"]
