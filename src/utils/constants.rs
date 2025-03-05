

use std::env;

use lazy_static::lazy_static;

lazy_static!{
    pub static ref ADDRESS : String = get_address();
    pub static ref PORT: u16 = get_port();
    pub static ref DATABASE_URL: String = get_database_url();
    pub static ref SECRET : String = get_secret();

}


fn get_address() -> String{
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap()
}


fn get_port() -> u16{
    dotenv::dotenv().ok();
    env::var("PORT").unwrap().parse::<u16>().unwrap()
}


fn get_database_url() -> String{
    dotenv::dotenv().ok();
    env::var("DATABASE_URL").unwrap()
}

fn get_secret() -> String{
    dotenv::dotenv().ok();
    env::var("SECRET").unwrap()
}