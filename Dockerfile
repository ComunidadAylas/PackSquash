FROM gcr.io/distroless/static-debian11@sha256:6d31326376a7834b106f281b04f67b5d015c31732f594930f2ea81365f99d60c

ARG TARGETARCH
COPY --chmod=755 packsquash-${TARGETARCH} /usr/bin/packsquash

WORKDIR /

ENTRYPOINT ["packsquash"]
