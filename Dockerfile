FROM gcr.io/distroless/static-debian11

ARG TARGETARCH
COPY --chmod=755 packsquash-${TARGETARCH} /usr/bin/packsquash

WORKDIR /

ENTRYPOINT ["packsquash"]
