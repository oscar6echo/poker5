
run-keygen:
    cargo run -p poker_keygen --release

test-poker:
    cargo test -p poker --lib --release

test-poker-vv:
    cargo test -p poker --lib --release -- --nocapture

test-server:
    cargo test -p poker_server --release

build:
    cargo build --release

show-keys:
    cargo run -p poker --bin show-keys --release

tactical:
    cargo run -p poker --bin tactical --release

server $RUST_LOG="info":
    cargo run -p poker_server --release

request-healthz:
    curl http://localhost:3000/healthz

request-config:
    curl http://localhost:3000/config

request-stats-five:
    curl http://localhost:3000/stats-five

request-stats-seven:
    curl http://localhost:3000/stats-seven

request-rank-five-sample:
    curl  -X POST -H "Content-Type: application/json" -d '{"hands":[[8,29,4,11,32],[9,30,5,12,33]]}' http://localhost:3000/rank-five

request-rank-seven-sample:
    curl  -X POST -H "Content-Type: application/json" -d '{"hands":[[8,29,4,11,32,18,19],[9,30,5,12,33,19,20]]}' http://localhost:3000/rank-seven

request-calc-det-sample-ok:
    curl -X POST -H "Content-Type: application/json" -d '{"players":[[8,29], [4,11]],"table":[]}' http://localhost:3000/calc-det

request-calc-det-sample-error-1:
    curl  -X POST -H "Content-Type: application/json" -d '{"players":[[8,29,18], [4,11]],"table":[20,21]}' http://localhost:3000/calc-det

request-calc-det-sample-error-2:
    curl  -X POST -H "Content-Type: application/json" -d '{"players":[[8,29], [4,11]],"table":[20,21]}' http://localhost:3000/calc-det

request-calc-mc-sample-1:
    curl -X POST -H "Content-Type: application/json" -d '{"players":[[8,9],[11],[]],"table":[15,47,23,33],"nb_game":100000}' http://localhost:3000/calc-mc

request-calc-mc-sample-2:
    curl -X POST -H "Content-Type: application/json" -d '{"players":[[8,9],[11],[]],"table":[15,47,23,33],"nb_game":100000000}' http://localhost:3000/calc-mc

doc-live-poker_keygen $PKG="poker_keygen":
    cargo watch --watch  $PKG -s 'cargo doc --no-deps -p $PKG'

doc-live-poker $PKG="poker":
    cargo watch --watch  $PKG -s 'cargo doc --no-deps -p $PKG'

doc-live-poker_server $PKG="poker_server":
    cargo watch --watch  $PKG -s 'cargo doc --no-deps -p $PKG'

serve-doc:
    browser-sync start --port 3009 --directory --server target/doc --ss target/doc --watch target/doc --no-open
