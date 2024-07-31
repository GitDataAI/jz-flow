mod channel_tracker;
mod mprc;
mod stream;

use jz_action::dbrepo::mongo::{MongoConfig, MongoRepo};
use jz_action::network::datatransfer::data_stream_server::DataStreamServer;
use jz_action::utils::StdIntoAnyhowResult;

use anyhow::{anyhow, Result};
use channel_tracker::ChannelTracker;
use clap::Parser;
use std::str::FromStr;
use std::sync::Arc;
use stream::ChannelDataStream;
use tokio::select;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tonic::transport::Server;
use tracing::{info, Level};

#[derive(Debug, Parser)]
#[command(
    name = "dp_runner",
    version = "0.0.1",
    author = "Author Name <github.com/GitDataAI/jz-action>",
    about = "embed in k8s images. make process data input output"
)]
struct Args {
    #[arg(short, long, default_value = "INFO")]
    log_level: String,

    #[arg(short, long)]
    node_name: String,

    #[arg(short, long)]
    mongo_url: String,

    #[arg(short, long)]
    database: String,

    #[arg(long, default_value = "0.0.0.0:80")]
    host_port: String,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_max_level(Level::from_str(&args.log_level)?)
        .try_init()
        .anyhow()?;

    let db_repo =
        Arc::new(MongoRepo::new(MongoConfig::new(args.mongo_url.clone()), &args.database).await?);

    let addr = args.host_port.parse()?;
    let program = ChannelTracker::new(db_repo.clone(), &args.node_name);
    let program_safe = Arc::new(Mutex::new(program));

    let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<Result<()>>(1);
    {
        let shutdown_tx = shutdown_tx.clone();
        let program_safe = program_safe.clone();
        let node_name = args.node_name.clone();
        let _ = tokio::spawn(async move {
            if let Err(err) =
                ChannelTracker::<Arc<MongoRepo>>::apply_db_state(db_repo, &node_name, program_safe)
                    .await
            {
                let _ = shutdown_tx
                    .send(Err(anyhow!("start data controller {err}")))
                    .await;
            }
        });
    }

    {
        //listen port
        let shutdown_tx_arc = shutdown_tx.clone();
        let _ = tokio::spawn(async move {
            let data_stream = ChannelDataStream {
                program: program_safe,
            };

            if let Err(e) = Server::builder()
                .add_service(DataStreamServer::new(data_stream))
                .serve(addr)
                .await
                .anyhow()
            {
                let _ = shutdown_tx_arc.send(Err(e)).await;
            }
        });

        info!("node listening on {}", addr);
    }

    {
        //catch signal
        let _ = tokio::spawn(async move {
            let mut sig_term = signal(SignalKind::terminate()).unwrap();
            let mut sig_int = signal(SignalKind::interrupt()).unwrap();
            select! {
                _ = sig_term.recv() => info!("Recieve SIGTERM"),
                _ = sig_int.recv() => info!("Recieve SIGTINT"),
            };
            let _ = shutdown_tx.send(Err(anyhow!("cancel by signal"))).await;
        });
    }

    shutdown_rx.recv().await;
    info!("gracefully shutdown");
    Ok(())
}
