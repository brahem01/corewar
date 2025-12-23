FROM debian:buster-slim

FROM debian:bullseye-slim

FROM debian:bookworm-slim

WORKDIR /corewar

RUN apt-get update && apt-get install -y bsdmainutils

COPY . . 

ENTRYPOINT ["/bin/bash"]


# FROM debian:bullseye-slim
# WORKDIR /corewar
# RUN apt-get update && apt-get install -y bsdextrautils util-linux

