use nntpbytes::connection::NewsConnection;
use nntpbytes::messages::auth::{AuthinfoMode, AuthinfoRequest};
use nntpbytes::server::NewsServer;

#[tokio::main]
async fn main() {
    let server: NewsServer = "example".try_into().unwrap();
    let mut connection = NewsConnection::connect(server, true).await.unwrap();
    println!("connected to {}", connection.fqdn());

    let user = AuthinfoRequest::new(AuthinfoMode::Username, "example");
    let response = connection.request(user).await.unwrap();

    println!("{} {}", response.code(), response.text());

    let pass = AuthinfoRequest::new(AuthinfoMode::Password, "example!");
    let response = connection.request(pass).await.unwrap();

    println!("{} {}", response.code(), response.text());
}
