use diesel::prelude::*;

use crate::db;
use crate::schema::{team_key_values, teams};
use diesel::deserialize::{self, FromSql};
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = SmallInt)]
pub enum TeamState {
    /// Can be attacked.
    Active,
    /// Registered but didn’t show up/stopped, don’t attack.
    Inactive,
    /// Erroneously created - hide, but keep for stats.
    Deleted,
}

impl ToSql<SmallInt, Pg> for TeamState {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let v = match *self {
            TeamState::Active => 1,
            TeamState::Inactive => 2,
            TeamState::Deleted => 3,
        };
        <i16 as ToSql<SmallInt, Pg>>::to_sql(&v, &mut out.reborrow())
    }
}

impl FromSql<SmallInt, Pg> for TeamState
where
    i16: FromSql<SmallInt, Pg>,
{
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        let v = i16::from_sql(bytes)?;
        Ok(match v {
            1 => TeamState::Active,
            2 => TeamState::Inactive,
            3 => TeamState::Deleted,
            id => return Err(format!("invalid team state id {}", id).into()),
        })
    }
}

#[derive(
    Identifiable, Insertable, Queryable, AsChangeset, Serialize, Deserialize, Eq, PartialEq, Debug,
)]
#[diesel(table_name = teams)]
pub struct Team {
    /// Team ID identifying the team in the CTF platform.
    id: i32,
    /// Team name for pretty printing.
    pub name: Option<String>,
    /// Should the team be attacked by default?
    pub state: TeamState,
}

/// Custom meta key/values which can be accessed in the template patterns.
#[derive(Identifiable, Queryable, AsChangeset, Associations, Serialize, Eq, PartialEq, Debug)]
#[diesel(table_name = team_key_values)]
#[diesel(primary_key(team_id, key))]
#[diesel(belongs_to(Team))]
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

    pub fn get_meta_data(&self, conn: &mut PgConnection) -> Result<Vec<TeamMeta>, db::Error> {
        Ok(TeamMeta::belonging_to(self)
            .load::<TeamMeta>(conn)?
            .into_iter()
            .collect::<Vec<TeamMeta>>())
    }

    pub fn save(&mut self, conn: &mut PgConnection) -> Result<(), db::Error> {
        diesel::update(&*self).set(&*self).execute(conn)?;
        Ok(())
    }

    pub fn set_name(&mut self, conn: &mut PgConnection, name: String) -> Result<(), db::Error> {
        self.name = Some(name);
        self.save(conn)
    }

    pub fn set_state(
        &mut self,
        conn: &mut PgConnection,
        state: TeamState,
    ) -> Result<(), db::Error> {
        self.state = state;
        self.save(conn)
    }
}

pub fn find_team_by_id(conn: &mut PgConnection, team_id: i32) -> Result<Option<Team>, db::Error> {
    use crate::schema::teams::dsl::*;

    let team = teams
        .filter(id.eq(team_id))
        .first::<Team>(conn)
        .optional()?;

    Ok(team)
}

pub fn get_teams(conn: &mut PgConnection) -> Result<Vec<Team>, db::Error> {
    use crate::schema::teams::dsl::*;
    Ok(teams.load::<Team>(conn)?)
}

pub fn add_team(conn: &mut PgConnection, team: Team) -> Result<(), db::Error> {
    use crate::schema::teams::dsl::*;

    diesel::insert_into(teams).values(&team).execute(conn)?;
    Ok(())
}
