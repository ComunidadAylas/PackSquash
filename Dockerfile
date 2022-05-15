FROM debian:bullseye-slim as build

ARG TARGETPLATFORM
FROM --platform=${TARGETPLATFORM} debian:bullseye-slim as copy_arm64
ONBUILD COPY --chmod=755 PackSquash-*-aarch64.AppImage /app/packsquash

ARG TARGETPLATFORM
FROM --platform=${TARGETPLATFORM} debian:bullseye-slim as copy_amd64
ONBUILD COPY --chmod=755 PackSquash-*-x86_64.AppImage /app/packsquash

ARG TARGETPLATFORM
ARG TARGETARCH
FROM --platform=${TARGETPLATFORM} copy_${TARGETARCH}

LABEL org.opencontainers.image.source="https://github.com/ComunidadAylas/PackSquash"
LABEL org.opencontainers.image.licenses=AGPL-3.0

WORKDIR /app
ENV PATH /app:$PATH

CMD ["packsquash", "--appimage-extract-and-run"]