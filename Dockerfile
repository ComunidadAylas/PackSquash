FROM gcr.io/distroless/static-debian11@sha256:046b92c933032a8ca99a66f4c79a68ac029d9a4ababd1a806a82140b3b899fd3

ARG TARGETARCH
COPY --chmod=755 packsquash-${TARGETARCH} /usr/bin/packsquash

WORKDIR /

ENTRYPOINT ["packsquash"]
