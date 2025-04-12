###############################################
#                 Build stage                 #
###############################################
FROM rust:1.85-alpine AS builder

# Set build arguments
ARG TARGETPLATFORM

# Install build dependencies
RUN apk add --no-cache musl-dev gcc

# Create a directory for the source code
WORKDIR /src

# Copy only the Cargo.toml and Cargo.lock first to leverage caching for dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy source file to allow `cargo build` to resolve dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only (this step will be cached unless dependencies change)
RUN cargo build --release || true

# Copy the actual source code
COPY . .

# Build the application based on the target platform
RUN cargo build --release && \
  mv ./target/release/rizzybox /bin/rizzybox

# Create symlinks for commands provided by rizzybox
RUN <<EOF
  for bin in $(rizzybox --list); do
    echo "ln -sf /bin/rizzybox /usr/local/bin/$bin"
    ln -sf /bin/rizzybox /usr/local/bin/$bin
  done
EOF

# Verify that the symlinks are created
RUN ls /usr/local/bin/sh || exit 1

###############################################
#                  App stage                  #
###############################################
FROM scratch

# Copy the built binary and symlinks from the builder stage
COPY --from=builder /bin/rizzybox /bin/rizzybox
COPY --from=builder /usr/local/bin /usr/local/bin

# Set the entrypoint and default command
ENTRYPOINT ["/bin/rizzybox"]
CMD ["sh"]
