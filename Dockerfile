FROM ubuntu:20.04

COPY target/release/dasmooikut_nl Rocket.toml /
COPY templates /templates
COPY public /public

ENTRYPOINT [ "/dasmooikut_nl" ]
