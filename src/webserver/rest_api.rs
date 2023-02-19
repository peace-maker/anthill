use crate::team;
use crate::DbPool;
use actix_web::{get, patch, put, web, Error, HttpResponse};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiError {
    pub error: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let rest_api = web::scope("/api")
        .service(get_teams)
        .service(get_team)
        .service(add_team)
        .service(update_team);

    cfg.service(rest_api);
}

#[derive(Deserialize)]
struct TeamArguments {
    include_meta_values: Option<bool>,
}

#[derive(Serialize)]
struct TeamResult {
    team: team::Team,
    meta_data: Option<Vec<team::TeamMeta>>,
}

#[get("/teams")]
async fn get_teams(
    pool: web::Data<DbPool>,
    args: web::Query<TeamArguments>,
) -> Result<HttpResponse, Error> {
    let team_list = web::block(move || -> Result<Vec<TeamResult>, crate::db::Error> {
        let conn = &mut pool.get()?;
        let db_team_list = team::get_teams(conn)?;

        let mut team_list = Vec::new();
        for db_team in db_team_list {
            let meta_data = if args.include_meta_values.unwrap_or(false) {
                Some(db_team.get_meta_data(conn)?)
            } else {
                None
            };
            team_list.push(TeamResult {
                team: db_team,
                meta_data,
            });
        }
        Ok(team_list)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(team_list))
}

#[get("/team/{team_id}")]
async fn get_team(
    pool: web::Data<DbPool>,
    args: web::Query<TeamArguments>,
    team_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let team_id = team_id.into_inner();
    let team = web::block(move || -> Result<Option<TeamResult>, crate::db::Error> {
        let conn = &mut pool.get()?;
        match team::find_team_by_id(conn, team_id)? {
            Some(team) => {
                let meta_data = if args.include_meta_values.unwrap_or(false) {
                    Some(team.get_meta_data(conn)?)
                } else {
                    None
                };
                Ok(Some(TeamResult { team, meta_data }))
            }
            None => Ok(None),
        }
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(team) = team {
        Ok(HttpResponse::Ok().json(team))
    } else {
        Ok(HttpResponse::NotFound().json(ApiError {
            error: format!("No team found with id: {team_id}"),
        }))
    }
}

#[put("/team")]
async fn add_team(
    pool: web::Data<DbPool>,
    team: web::Json<team::Team>,
) -> Result<HttpResponse, Error> {
    let team = web::block(move || {
        let conn = &mut pool.get()?;
        team::add_team(conn, team.into_inner())
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(team))
}

#[patch("/team/{team_id}")]
async fn update_team(
    pool: web::Data<DbPool>,
    team_id: web::Path<i32>,
    new_team: web::Json<team::Team>,
) -> Result<HttpResponse, Error> {
    let team_id = team_id.into_inner();
    let new_team = new_team.into_inner();
    let team = web::block(move || -> Result<Option<team::Team>, crate::db::Error> {
        let conn = &mut pool.get()?;
        match team::find_team_by_id(conn, team_id)? {
            Some(mut team) => {
                team.name = new_team.name.clone();
                team.state = new_team.state;
                team.save(conn)?;
                Ok(Some(team))
            }
            None => Ok(None),
        }
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(team) = team {
        Ok(HttpResponse::Ok().json(team))
    } else {
        Ok(HttpResponse::NotFound().json(ApiError {
            error: format!("No team found with id: {team_id}"),
        }))
    }
}
