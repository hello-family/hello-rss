use envconfig::Envconfig;

#[derive(Envconfig, Clone, Debug)]
pub struct Config {
    #[envconfig(from = "DATABASE_URL", default = "sqlite:data.db?mode=rwc")]
    pub db_url: String,

    #[envconfig(from = "SERVER_URL", default = "127.0.0.1:3000")]
    pub server_url: String,

    #[envconfig(from = "JWT_SECRET")]
    pub jwt_secret: String,
}

lazy_static! {
    pub static ref APP_CONFIG: &'static Config = &Config::init_from_env().unwrap();
}
