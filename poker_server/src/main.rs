use axum::{
    extract::{rejection::JsonRejection, FromRequest, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
    vec,
};

use tracing_subscriber;

use poker_eval::{
    calc::{
        self,
        equity_det::{GameError, HandEquity},
        equity_mc::McGameError,
    },
    eval::{self, five::get_rank_five, seven::get_rank, target::HandStats},
    stats,
};

/// ## Poker eval config
/// Converts cards in human readable format to integer and vice versa.
#[derive(Debug, Serialize)]
struct Config {
    /// faces as char: 2, 3, 4, 5, 6, 7, 8, 9, T, J, Q, K, A
    face: [char; 13],
    /// suits as char: C, D, H, S
    suit: [char; 4],
    /// dict of card 2-char string to card number, e.g. "2C" -> 0, "2D" -> 1, ..., "AS" -> 51
    card_no: HashMap<String, usize>,
    /// dict of card number to 2-char string, e.g. 0 -> "2C", 1 -> "2D", ..., 51 -> "AS"
    card_sy: HashMap<usize, String>,
}

/// ## Poker eval app state
/// Contains the tables and hand statistics for the poker server.
/// Calculated at server start.
#[derive(Clone, Debug, Serialize)]
struct AppState {
    /// 5-card and 7-card lookup tables used by poker_eval
    #[serde(skip)]
    t7: Arc<eval::seven::TableSeven>,
    /// 5-card hand statistics
    stats_five: HashMap<String, HandStats>,
    /// 7-card hand statistics
    stats_seven: HashMap<String, HandStats>,
}

/// ## Poker eval server config
/// Contains the server address and port.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for poker_eval.")]
struct ServerConfig {
    /// IP address to listen on, default is 127.0.0.1
    #[clap(short = 'a', long = "addr", default_value = "127.0.0.1")]
    addr: String,
    /// Port to listen on, default is 3000
    #[clap(short = 'p', long = "port", default_value = "3000")]
    port: u16,
}

/// ## 5-card hands
#[derive(Debug, Deserialize)]
struct HandsFive {
    /// list of 5-card hands
    hands: Vec<[usize; 5]>,
}

/// ## 7-card hands
#[derive(Debug, Deserialize)]
struct HandsSeven {
    /// list of 7-card hands
    hands: Vec<[usize; 7]>,
}

/// ## Deterministic game description
/// Contains all players and the table cards.
#[derive(Debug, Deserialize)]
struct GameDet {
    /// list of players with their 2 cards
    players: Vec<[u32; 2]>,
    /// table cards
    table: Vec<u32>,
}

/// ## Monte Carlo game description
/// Contains all known (or assumed) players cards and the table cards.
#[derive(Debug, Deserialize)]
struct GameMc {
    /// list of players with their known cards (0, 1, or 2)
    players: Vec<Vec<u32>>,
    /// table cards
    table: Vec<u32>,
    /// number of games to simulate
    nb_game: u32,
}

/// ## Start poker server
/// Starts the poker server based on [poker_eval] crate.  
/// First build app state with lookup tables and hand statistics.  
/// Then start the server with the app state.  
#[tokio::main]
async fn main() {
    banner("poker server", 10);

    // read args
    let server_config = ServerConfig::parse();
    let sock_addr = SocketAddr::from((
        IpAddr::from_str(server_config.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        server_config.port,
    ));

    // tracing global collector configured based on RUST_LOG env var.
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poker_server=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    // init app state
    let state = build_app_state();

    // create app
    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/config", get(config))
        .route("/stats-five", get(stats_five))
        .route("/stats-seven", get(stats_seven))
        .route("/rank-five", post(rank_five))
        .route("/rank-seven", post(rank_seven))
        .route("/calc-det", post(calc_det))
        .route("/calc-mc", post(calc_mc))
        .with_state(state);

    // start server
    let listener = tokio::net::TcpListener::bind(sock_addr).await.unwrap();
    tracing::info!("Listening on {}", sock_addr);
    axum::serve(listener, app).await.unwrap();
}

/// ## Init function
/// Builds the app state at server start.  
/// Contains all lookup tables used by poker_eval and hand statistics for `/stats-five` and  `/stats-seven` endpoints.
fn build_app_state() -> AppState {
    let start = std::time::Instant::now();

    let t7 = eval::seven::build_tables(false);
    let stats_seven = stats::build_seven(t7.clone(), false);

    let t5_ = Arc::new(t7.t5.clone());
    let stats_five = stats::build_five(t5_, false);

    let end = std::time::Instant::now();
    tracing::info!("build_app_state runtime = {:?} s", end - start);

    AppState {
        t7,
        stats_five,
        stats_seven,
    }
}

/// ## Health check endpoint
/// Returns "Ok" if the server is running.  
#[tracing::instrument]
async fn healthz() -> &'static str {
    tracing::info!("-> OK");
    "Ok"
}

/// ## Config endpoint
/// Returns the poker eval Config object.  
#[tracing::instrument(skip(state))]
async fn config(State(state): State<AppState>) -> (StatusCode, Json<Config>) {
    let t7_ = state.t7.clone();

    let conf = Config {
        face: t7_.t5.pk.face,
        suit: t7_.t5.pk.suit,
        card_no: t7_.t5.pk.card_no.clone(),
        card_sy: t7_.t5.pk.card_sy.clone(),
    };

    tracing::info!("-> {:?}", conf);
    (StatusCode::OK, Json(conf))
}

/// ## 5-card hand statistics endpoint
/// Returns the 5-card hand statistics.
#[tracing::instrument(skip(state))]
async fn stats_five(
    State(state): State<AppState>,
) -> (StatusCode, Json<HashMap<String, HandStats>>) {
    let stats_five = state.stats_five.clone();

    tracing::info!("-> {:?}", stats_five);
    (StatusCode::OK, Json(stats_five))
}

/// ## 7-card hand statistics endpoint
/// Returns the 7-card hand statistics.
#[tracing::instrument(skip(state))]
async fn stats_seven(
    State(state): State<AppState>,
) -> (StatusCode, Json<HashMap<String, HandStats>>) {
    let stats_seven = state.stats_seven.clone();

    tracing::info!("-> {:?}", stats_seven);
    (StatusCode::OK, Json(stats_seven))
}

/// ## 5-card hand rank endpoint
/// Returns the rank of each 5-card hand passed as input.
#[tracing::instrument(skip(state))]
async fn rank_five(
    State(state): State<AppState>,
    Json(payload): Json<HandsFive>,
) -> (StatusCode, Json<Vec<u32>>) {
    let t5_ = &state.t7.t5;

    let hands = payload.hands;
    let mut ranks = vec![];

    for hand in hands.iter() {
        let rank = get_rank_five(t5_, *hand);
        ranks.push(rank);
    }

    tracing::info!("-> ranks={:?}", ranks);
    (StatusCode::OK, Json(ranks))
}

/// ## 7-card hand rank endpoint
/// Returns the rank of each 7-card hand passed as input.
#[tracing::instrument(skip(state))]
async fn rank_seven(
    State(state): State<AppState>,
    Json(payload): Json<HandsSeven>,
) -> (StatusCode, Json<Vec<u32>>) {
    let t7_ = state.t7;

    let hands = payload.hands;
    let mut ranks = vec![];

    for hand in hands.iter() {
        let rank = get_rank(&t7_, *hand);
        ranks.push(rank);
    }

    tracing::info!("-> ranks={:?}", ranks);
    (StatusCode::OK, Json(ranks))
}

/// ## Deterministic hand equity calculation
/// Calculates the equity of each player hand with all players and table cards known.  
/// Exhaustive calculation through all possible table unknown cards.  
#[tracing::instrument(skip(state))]
async fn calc_det(
    State(state): State<AppState>,
    AppJson(payload): AppJson<GameDet>,
) -> Result<AppJson<Vec<HandEquity>>, AppError> {
    let t7_ = state.t7.clone();

    let equity = calc::equity_det::calc_equity_det(
        t7_,
        payload.players.clone(),
        payload.table.clone(),
        false,
    )?;

    tracing::info!("-> equity={:?}", equity);

    Ok(AppJson(equity))
}

/// ## Monte Carlo hand equity calculation  
/// Calculates the equity of the first player hand with partial information about other players hands and table cards known.  
/// Monte Carlo simulation through `nb_game` games.  
#[tracing::instrument(skip(state))]
async fn calc_mc(
    State(state): State<AppState>,
    AppJson(payload): AppJson<GameMc>,
) -> Result<AppJson<HandEquity>, AppError> {
    let t7_ = state.t7.clone();

    let equity = calc::equity_mc::calc_equity_monte_carlo(
        t7_,
        payload.players.clone(),
        payload.table.clone(),
        payload.nb_game,
    )?;

    tracing::info!("-> equity={:?}", equity);

    Ok(AppJson(equity))
}

/// ## Util function
/// Prints a banner.
fn banner(txt: &str, n: u8) {
    let s = "-".repeat(n as usize);
    println!("\n{} {} {}", s, txt, s);
}

// --------------------
// --------------------
// --------------------
// error handling

/// Wrapper around `axum::Json` that implements `IntoResponse` for `AppJson`.
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

/// Error type for the app.
enum AppError {
    /// Rejection from `axum::Json`.
    JsonRejection(JsonRejection),
    /// Error from the poker_eval lib.
    GameError(GameError),
    /// Error from the poker_eval lib.
    McGameError(McGameError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            AppError::JsonRejection(rejection) => {
                tracing::error!("bad user input -> {:?}", rejection.body_text());
                // println!("bad user input -> {:?}", rejection);
                (rejection.status(), rejection.body_text())
            }
            AppError::GameError(err) => {
                tracing::error!("error from poker lib -> {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))
            }
            AppError::McGameError(err) => {
                tracing::error!("error from poker lib -> {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))
            }
        };

        (status, AppJson(ErrorResponse { message })).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        // transparent
        Self::JsonRejection(rejection)
    }
}

impl From<GameError> for AppError {
    fn from(error: GameError) -> Self {
        // transparent
        Self::GameError(error)
    }
}

impl From<McGameError> for AppError {
    fn from(error: McGameError) -> Self {
        // transparent
        Self::McGameError(error)
    }
}
