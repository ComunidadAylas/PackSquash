FROM gcr.io/distroless/static-debian11@sha256:a43abc840a7168c833a8b3e4eae0f715f7532111c9227ba17f49586a63a73848

ARG TARGETARCH
COPY --chmod=755 packsquash-${TARGETARCH} /usr/bin/packsquash

WORKDIR /

ENTRYPOINT ["packsquash"]
