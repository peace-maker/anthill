use diesel::prelude::*;

use serde::{Deserialize, Serialize};
use diesel::sql_types::*;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, ToSql, Output};
use crate::schema::{teams, team_key_values};

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize, AsExpression, FromSqlRow)]
pub enum TeamState {
    /// Can be attacked.
    Active,
    /// Registered but didn’t show up/stopped, don’t attack.
    Inactive,
    /// Erroneously created - hide, but keep for stats.
    Deleted,
}

impl<DB: Backend> ToSql<SmallInt, DB> for TeamState
where
    i16: ToSql<SmallInt, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: std::io::Write,
    {
        let v = match *self {
            TeamState::Active => 1,
            TeamState::Inactive => 2,
            TeamState::Deleted => 3,
        };
        v.to_sql(out)
    }
}

impl<DB: Backend> FromSql<SmallInt, DB> for TeamState
where
    i16: FromSql<SmallInt, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let v = i16::from_sql(bytes)?;
        Ok(match v {
            1 => TeamState::Active,
            2 => TeamState::Inactive,
            3 => TeamState::Deleted,
            _ => return Err("replace me with a real error".into()),
        })
    }
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, PartialEq, Debug)]
#[table_name = "teams"]
pub struct Team {
    /// Team ID identifying the team in the CTF platform.
    id: i32,
    /// Team name for pretty printing.
    name: Option<String>,
    /// Custom meta key/values which can be accessed in the template patterns.
    //meta: HashMap<String, String>,
    /// Should the team be attacked by default?
    state: TeamState,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "team_key_values"]
#[primary_key(team_id)]
#[belongs_to(Team)]
pub struct TeamMeta {
    team_id: i32,
    key: String,
    value: String,
}

impl Team {
    /// Should this team be targetted by exploits by default?
    pub fn should_attack(&self) -> bool {
        self.state == TeamState::Active
    }
}

pub fn find_team_by_id(
    team_id: i32,
    conn: &PgConnection,
) -> Result<Option<Team>, DbError> {
    use crate::schema::teams::dsl::*;

    let team = teams
        .filter(id.eq(team_id))
        .first::<Team>(conn)
        .optional()?;
    
    // if let Some(ref team) = team {
    //     let team_meta = TeamMeta::belonging_to(team).load::<TeamMeta>(conn);

    // }

    Ok(team)
}

pub fn get_teams(
    conn: &PgConnection,
) -> Result<Option<Vec<Team>>, DbError> {
    use crate::schema::teams::dsl::*;

    let team_list = teams
        .load::<Team>(conn)
        .optional()?;

    Ok(team_list)
}