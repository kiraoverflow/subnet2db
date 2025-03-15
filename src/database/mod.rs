use diesel::PgConnection;
use diesel::r2d2;
use dotenv_codegen::dotenv;

type PgPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
type StdErr = Box<dyn std::error::Error>;
#[derive(Clone)]


pub struct Db {
    pub pool: PgPool,
}

impl Db {
    pub fn connect() -> Result<Self, StdErr> {
        let db_url = dotenv!("DATABASE_URL");
        let manager = r2d2::ConnectionManager::new(db_url);
        let pool = r2d2::Pool::new(manager)?;
        Ok(Db { pool })
    }
}


