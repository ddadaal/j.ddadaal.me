# ==================================================================
# Stage: Build
# ==================================================================
FROM clux/muslrust:1.40.0-stable as build

# Change workdir
WORKDIR "/app"

# Copy the files
COPY . .

# Compile
RUN cargo build --target x86_64-unknown-linux-musl --release

# ==================================================================
# Stage: Run
# ==================================================================
FROM alpine:latest as run

RUN apk --no-cache add ca-certificates

# Copy compiled binary to /app
COPY --from=build "/app/target/x86_64-unknown-linux-musl/release/j-ddadaal-me" "/app/"

WORKDIR "/app"

# ENTRYPOINT ["j-ddadaal-me"]
