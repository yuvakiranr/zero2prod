use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:300").await?;
    run(listener)?.await
}
