use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws::{self, CloseCode, CloseReason};

use super::rest_api::ApiError;
use crate::team;
use crate::DbPool;
use serde::Deserialize;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct WsApiSession {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,

    pool: DbPool,
}

#[derive(Debug)]
struct WsApiError {
    error: String,
}
impl std::fmt::Display for WsApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}
impl std::error::Error for WsApiError {}

#[derive(Deserialize)]
struct WsApiCommandGetTeam {
    team_id: i32,
}

impl WsApiSession {
    pub fn new(pool: DbPool) -> Self {
        Self {
            hb: Instant::now(),
            pool,
        }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }

    fn handle_command(
        &mut self,
        ctx: &mut <WsApiSession as Actor>::Context,
        message: &str,
    ) -> Result<(), crate::db::Error> {
        let command: ApiCommand = serde_json::from_str(message)?;

        match command.cmd.as_str() {
            "teams" => {
                let conn = &mut self.pool.get()?;
                let team_list = team::get_teams(conn)?;
                ctx.text(serde_json::to_string(&team_list).unwrap());
            }
            "get_team" => {
                let command: WsApiCommandGetTeam = serde_json::from_str(message)?;
                let conn = &mut self.pool.get()?;
                let team = team::find_team_by_id(conn, command.team_id)?;

                if let Some(team) = team {
                    ctx.text(serde_json::to_string(&team).unwrap());
                } else {
                    ctx.text(
                        serde_json::to_string(&ApiError {
                            error: format!("No team found with id: {}", command.team_id),
                        })
                        .unwrap(),
                    );
                }
            }
            _ => {
                let error = ApiError {
                    error: format!("Unknown command: {}", command.cmd),
                };
                ctx.text(serde_json::to_string(&error).unwrap());
                log::error!("{}", error.error);
                return Err(Box::new(WsApiError { error: error.error }));
            }
        }
        Ok(())
    }
}

impl Actor for WsApiSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

#[derive(Deserialize, Debug)]
struct ApiCommand {
    cmd: String,
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsApiSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        // process websocket messages
        // println!("WS: {msg:?}");
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let message = text.trim();
                if let Err(err) = self.handle_command(ctx, message) {
                    log::error!("Error: {} ({})", message, err);
                    ctx.text("Error");
                    ctx.close(Some(CloseReason::from(CloseCode::Error)));
                    ctx.stop();
                }
            }
            ws::Message::Binary(_) => log::error!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
