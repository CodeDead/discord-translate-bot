# Use the latest Rust image to build the application
FROM rust:latest AS build

# Set the working directory
WORKDIR /usr/src/discord_translate_bot

# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY Cargo.toml Cargo.lock ./

# Copy the rest of the application code to the container
COPY src ./src

# Build the application
RUN cargo build --release

# Start a new stage for the runtime image
FROM debian:stable-slim

# Install required dependencies
RUN apt-get update && apt install -y openssl

# Set the environment variables
ENV GOOGLE_TRANSLATE_TOKEN=your_google_translate_token
ENV DISCORD_TOKEN=your_discord_token

# Set the working directory
WORKDIR /usr/src/discord_translate_bot

# Copy the binary from the build stage to the current stage
COPY --from=build /usr/src/discord_translate_bot/target/release/discord_translate_bot .

# Set the binary as the entrypoint of the container
CMD ["./discord_translate_bot"]
