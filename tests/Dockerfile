FROM alpine:3

RUN adduser -D testuser \
    && addgroup -S sudoers

ENTRYPOINT ["/sbin/tini", "--"]
