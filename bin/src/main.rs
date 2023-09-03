use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use hyper::Server;
use service::config::Config;
use service::id::IdGenerator;

use controller::MakeSvc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let raw_config = std::fs::read_to_string("./config.yaml")?;
    let config: Config = serde_yaml::from_str(&raw_config)?;

    log::info!("running service with config: {:?}", config);

    let address = SocketAddr::from_str(&config.address)?;

    let id_gen = kono::KonoIdGenerator::<kono::RocksDB>::new_with_rocksdb(
        config.rocksdb_path.clone().unwrap(),
        config.increment,
        config.starting,
        config.sharding_bits,
        config.rocksdb_storage_key.clone().unwrap(),
    );
    let mut service = IdGenerator::new();
    service.start(id_gen, config.clone());

    let make_svc = MakeSvc::new(Arc::new(service.sender().unwrap()));
    let server = Server::bind(&address).serve(make_svc);
    server.await?;

    Ok(())
}
