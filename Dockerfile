# This assumes binaries are present, see COPY directive.

ARG IMGNAME=gcr.io/distroless/cc

FROM alpine AS chmodder
ARG TARGETARCH
COPY /artifacts/binaries-$TARGETARCH/beam-inspector /app/inspector
RUN chmod +x /app/*

FROM ${IMGNAME}
COPY --from=chmodder /app/inspector /usr/local/bin/
ENTRYPOINT [ "/usr/local/bin/inspector" ]
