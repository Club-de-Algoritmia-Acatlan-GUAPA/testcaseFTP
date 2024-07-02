use libunftp::Server;
use testcaseFTP::configuration::get_configuration;
use unftp_sbe_fs::ServerExt;

#[tokio::main]
pub async fn main() {
    let config = get_configuration().unwrap();
    let server = Server::with_fs(config.ftp_home)
        .greeting("Welcome to my Juez guapa test server")
        .passive_ports(50000..65535)
        .build()
        .unwrap();
    let ftp_url = format!("{}:{}", config.ftp.host, config.ftp.port);
    println!("Starting FTP server on {}", ftp_url);
    match server.listen(ftp_url).await {
        Ok(_) => eprintln!(
            "FTP server started on ftp://{}:{}",
            config.ftp.host, config.ftp.port
        ),
        Err(e) => eprintln!("Failed to start FTP server: {}", e),
    }
}
