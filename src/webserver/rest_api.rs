use actix_web::{get, web, Error, HttpResponse};
use serde::Serialize;
use crate::team;
use crate::DbPool;

#[derive(Serialize)]
pub struct ApiError {
    pub error: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let rest_api = web::scope("/api")
    .service(get_teams)
    .service(get_team);

    cfg.service(rest_api);
}

#[get("/teams")]
async fn get_teams(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let team_list = web::block(move || {
        let conn = pool.get()?;
        team::get_teams(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(team_list) = team_list {
        Ok(HttpResponse::Ok().json(team_list))
    } else {
        let res = HttpResponse::NotFound().json(ApiError { error: "Failed to fetch team list.".to_string() });
        Ok(res)
    }
}

#[get("/team/{team_id}")]
async fn get_team(
    pool: web::Data<DbPool>, 
    team_id: web::Path<i32>
) -> Result<HttpResponse, Error> {
    let team_id = team_id.into_inner();
    let team = web::block(move || {
        let conn = pool.get()?;
        team::find_team_by_id(team_id, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(team) = team {
        Ok(HttpResponse::Ok().json(team))
    } else {
        let res = HttpResponse::NotFound().json(ApiError { error: format!("No team found with id: {team_id}") });
        Ok(res)
    }
}