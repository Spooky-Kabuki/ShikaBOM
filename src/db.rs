use postgres::{Client, NoTls};

pub fn postgres_init() -> Client {
    return Client::connect("host=localhost user=rootben password=password dbname=shika", NoTls).unwrap();
}