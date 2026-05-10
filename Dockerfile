FROM gcr.io/distroless/static-debian13@sha256:6f3f2123de90d2e7998b8161a2838433ec32560a827d07bcab339dacbf0cf16f

ARG TARGETARCH
COPY --chmod=755 packsquash-${TARGETARCH} /usr/bin/packsquash

WORKDIR /

ENTRYPOINT ["packsquash"]
