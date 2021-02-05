FROM debian:buster-20201012-slim AS builder

WORKDIR /

# NOTE we need to add "-y" as an arg. while we're at it set default toolchain to nightly for rocket
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -y
COPY Cargo.lock Cargo.toml /
COPY src /src
COPY scripts /scripts
RUN $HOME/.cargo/bin/cargo build --release
RUN scripts/download-and-import.sh

# ---

FROM debian:buster-20201012-slim
ENV TZ=Europe/Amsterdam
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

COPY Rocket.toml /
COPY templates /templates
COPY public /public

# Built in prev stage
COPY --from=builder target/release/dasmooikut_nl /

CMD [ "/dasmooikut_nl" ]
