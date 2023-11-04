FROM solanalabs/rust

ENV USER=appuser
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /user/src/app
COPY . .

RUN sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
ENV PATH="/root/.local/share/solana/install/active_release/bin:$PATH"

#CMD sh -c "cargo build-bpf --manifest-path=./solana_test_lib/Cargo.toml --bpf-out-dir=./solana_test_lib/src/dist/program"