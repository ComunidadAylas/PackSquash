FROM debian:bullseye-slim

LABEL org.opencontainers.image.source="https://github.com/ComunidadAylas/PackSquash"
LABEL org.opencontainers.image.licenses=AGPL-3.0

COPY --chmod=755 PackSquash-*-x86_64.AppImage /app/packsquash
WORKDIR /app
ENV PATH /app:$PATH

CMD ["packsquash", "--appimage-extract-and-run"]
