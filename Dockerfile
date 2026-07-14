FROM gcr.io/distroless/static-debian13@sha256:c0e338684f4271e71aace102225a72650376f64452cb24135c343a221fa54d3b

ARG TARGETARCH
COPY --chmod=755 packsquash-${TARGETARCH} /usr/bin/packsquash

WORKDIR /

ENTRYPOINT ["packsquash"]
