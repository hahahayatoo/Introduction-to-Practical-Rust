use actix_web::{HttpResponse, State};
use failure::Error;

use crate::Server;

use actix_web::{Json, Query};
use log::debug;

/// POST /csv のハンドラ
pub fn handle_post_csv(server: State<Server>) -> Result<HttpResponse, Error> {
    // レスポンスはDefaultでダミーを生成
    let logs = Default::default();

    Ok(HttpResponse::Ok().json(api::csv::post::Response(logs)))
}

/// POST /logs のハンドラ
pub fn handle_post_logs(
    server: State<Server>,
    log: Json<api::logs::post::Request>, // POSTのボディはJson<T>を引数に書くと自動的にデシリアライズされて渡される
) -> Result<HttpResponse, Error> {
    // Json<T>はTへのDerefを実装しているので内部はほぼそのままTの値として扱える
    debug!("{:?}", log);
    // レスポンスはAccepted
    Ok(HttpResponse::Accepted().finish())
}

/// GET /log のハンドラ
pub fn handle_get_logs(
    server: State<Server>,
    // クエリパラメータはQuery<T>を引数に書くと自動的にデシリアライズして渡される
    range: Query<api::logs::get::Query>,
) -> Result<HttpResponse, Error> {
    debug!("{:?}", range);
    let logs = Default::default();

    Ok(HttpResponse::Ok().json(api::logs::get::Response(logs)))
}

/// GET /csv のハンドラ
pub fn handle_get_csv(
    server: State<Server>,
    range: Query<api::csv::get::Query>,
) -> Result<HttpResponse, Error> {
    debug!("{:?}", range);

    // CSVファイルはバイナリデータにして返す
    let csv: Vec<u8> = vec![];
    Ok(HttpResponse::Ok()
        .header("Content-Type", "text/csv")
        .body(csv))
}