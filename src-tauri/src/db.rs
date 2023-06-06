use postgres::{Client, NoTls};

pub fn postgres_init() -> Client {
    return Client::connect("host=lapras.dex user=rootben password=password dbname=nudb", NoTls).unwrap();
}