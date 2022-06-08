FROM debian:bullseye-slim AS libs

# Use the base variant because we need glibc components
FROM gcr.io/distroless/base-debian11

ARG TARGETARCH
COPY --chmod=755 PackSquash-*-${TARGETARCH}.AppImage /usr/bin/packsquash

# Copy runtime dependency from the base Debian zlib1g package.
# The AppImage runtime assumes that it is present
COPY --from=libs /lib/*-linux-gnu/libz.so.1 /lib/libz.so.1

WORKDIR /

ENTRYPOINT ["packsquash", "--appimage-extract-and-run"]
