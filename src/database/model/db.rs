use diesel::{r2d2::ConnectionManager, PgConnection};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
