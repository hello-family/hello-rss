use envconfig::Envconfig;
use tokio::sync::OnceCell;

#[derive(Envconfig, Clone, Debug)]
pub struct Config {
    #[envconfig(from = "DATABASE_URL", default = "sqlite:data.db?mode=rwc")]
    pub db_url: String,

    #[envconfig(from = "SERVER_URL", default = "127.0.0.1:3000")]
    pub server_url: String,

    #[envconfig(from = "JWT_SECRET")]
    pub jwt_secret: String,
}

impl Config {
    pub async fn get() -> &'static Config {
        static CONFIG: OnceCell<Config> = OnceCell::const_new();
        CONFIG.get_or_init(Config::init).await
    }

    async fn init() -> Config {
        Config::init_from_env().expect("Load config from env error")
    }
}
