FROM rust:1.60-buster as builder

RUN USER=root cargo new --bin ocs
WORKDIR ./ocs
COPY ./Cargo.toml ./Cargo.toml
COPY ./.env ./.env
COPY ./openeuler/config  /usr/local/cargo

RUN CARGO_HTTP_MULTIPLEXING=false cargo fetch

RUN cargo build --release \
  && rm src/*.rs target/release/deps/ocs*

ADD . ./

RUN cargo build --release


FROM openeuler/openeuler:22.03-lts

ARG APP=/usr/src/app

RUN dnf update \
  && dnf install -y ca-certificates tzdata \
  && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

ENV TZ=Etc/UTC \
  APP_USER=appuser

RUN groupadd $APP_USER \
  && useradd -g $APP_USER $APP_USER \
  && mkdir -p ${APP}

COPY --from=builder /ocs/target/release/ocs ${APP}/ocs
COPY --from=builder ocs/.env ${APP}/.env


RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./ocs"]