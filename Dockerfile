FROM phusion/baseimage:0.11 AS builder
LABEL maintainer="jake@commonwealth.im"
LABEL description="This is the build stage. Here we create the binary."

ARG PROFILE=release
WORKDIR /straightedge

COPY . /straightedge
RUN /straightedge/setup.sh

# ===== SECOND STAGE ======

FROM phusion/baseimage:0.11
LABEL maintainer="hello@commonwealth.im"
LABEL description="This is the 2nd stage: a very small image where we copy the Straightedge binary."
ARG PROFILE=release
COPY --from=builder /straightedge/target/$PROFILE/straightedge /usr/local/bin
COPY --from=builder /straightedge/testnets /usr/local/bin/testnets

RUN rm -rf /usr/lib/python* && \
	mkdir -p /root/.local/share && \
	ln -s /root/.local/share /data

EXPOSE 30333 9933 9944
VOLUME ["/data"]

WORKDIR /usr/local/bin
CMD ["straightedge", "--chain", "straightedge", "--ws-external"]
