use diesel::{Connection, ConnectionResult, PgConnection};

use crate::state::CONFIG_STATES;

pub fn get_connection() -> ConnectionResult<PgConnection> {
    PgConnection::establish(&CONFIG_STATES.postgre_url)
}