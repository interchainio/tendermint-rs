//! CLI for performing simple interactions against a Tendermint node's RPC.

use futures::StreamExt;
use std::convert::TryInto;
use std::str::FromStr;
use std::time::Duration;
use structopt::StructOpt;
use tendermint::abci::{Path, Transaction};
use tendermint_rpc::query::Query;
use tendermint_rpc::{
    Client, Error, HttpClient, Order, Paging, Result, Scheme, Subscription, SubscriptionClient,
    Url, WebSocketClient,
};
use tracing::level_filters::LevelFilter;
use tracing::{error, info, warn};

/// CLI for performing simple interactions against a Tendermint node's RPC.
///
/// Supports HTTP, HTTPS, WebSocket and secure WebSocket (wss://) URLs.
#[derive(Debug, StructOpt)]
struct Opt {
    /// The URL of the Tendermint node's RPC endpoint.
    #[structopt(
        short,
        long,
        default_value = "http://127.0.0.1:26657",
        env = "TENDERMINT_RPC_URL"
    )]
    url: Url,

    /// An optional HTTP/S proxy through which to submit requests to the
    /// Tendermint node's RPC endpoint. Only available for HTTP/HTTPS endpoints
    /// (i.e. WebSocket proxies are not supported).
    #[structopt(long)]
    proxy_url: Option<Url>,

    /// Increase output logging verbosity to DEBUG level.
    #[structopt(short, long)]
    verbose: bool,

    #[structopt(subcommand)]
    req: Request,
}

#[derive(Debug, StructOpt)]
enum Request {
    #[structopt(flatten)]
    ClientRequest(ClientRequest),
    /// Subscribe to receive events produced by a specific query.
    Subscribe {
        /// The query against which events will be matched.
        query: Query,
        /// The maximum number of events to receive before terminating.
        #[structopt(long)]
        max_events: Option<u32>,
        /// The maximum amount of time (in seconds) to listen for events before
        /// terminating.
        #[structopt(long)]
        max_time: Option<u32>,
    },
}

#[derive(Debug, StructOpt)]
enum ClientRequest {
    /// Request information about the ABCI application.
    AbciInfo,
    /// Query the ABCI application.
    AbciQuery {
        /// The path for which you want to query, if any.
        #[structopt(long)]
        path: Option<String>,
        /// The data for which you want to query.
        data: String,
        /// The block height at which to query.
        #[structopt(long)]
        height: Option<u32>,
        #[structopt(long)]
        prove: bool,
    },
    /// Get a block at a given height.
    Block { height: u32 },
    /// Get block headers between two heights (min <= height <= max).
    Blockchain {
        /// The minimum height
        min: u32,
        /// The maximum height.
        max: u32,
    },
    /// Request the block results at a given height.
    BlockResults {
        /// The height of the block you want.
        height: u32,
    },
    // TODO(thane): Implement evidence broadcast
    /// Broadcast a transaction asynchronously (without waiting for the ABCI
    /// app to check it or for it to be committed).
    BroadcastTxAsync {
        /// The transaction to broadcast.
        tx: String,
    },
    /// Broadcast a transaction, waiting for it to be fully committed before
    /// returning.
    BroadcastTxCommit {
        /// The transaction to broadcast.
        tx: String,
    },
    /// Broadcast a transaction synchronously (waiting for the ABCI app to
    /// check it, but not for it to be committed).
    BroadcastTxSync {
        /// The transaction to broadcast.
        tx: String,
    },
    /// Get the commit for the given height.
    Commit { height: u32 },
    /// Get the current consensus state.
    ConsensusState,
    /// Get the node's genesis data.
    Genesis,
    /// Get the node's health.
    Health,
    /// Request the latest block.
    LatestBlock,
    /// Request the results for the latest block.
    LatestBlockResults,
    /// Request the latest commit.
    LatestCommit,
    /// Obtain information about the P2P stack and other network connections.
    NetInfo,
    /// Get Tendermint status (node info, public key, latest block hash, etc.).
    Status,
    /// Search for transactions with their results.
    TxSearch {
        /// The query against which transactions should be matched.
        query: Query,
        #[structopt(long, default_value = "1")]
        page: u32,
        #[structopt(long, default_value = "10")]
        per_page: u8,
        #[structopt(long, default_value = "asc")]
        order: Order,
        #[structopt(long)]
        prove: bool,
    },
    /// Get the validators at the given height.
    Validators {
        /// The height at which to query the validators.
        height: u32,
        /// Fetch all validators.
        #[structopt(long)]
        all: bool,
        /// The page of validators to retrieve.
        #[structopt(long)]
        page: Option<usize>,
        /// The number of validators to retrieve per page.
        #[structopt(long)]
        per_page: Option<u8>,
    },
}

#[tokio::main]
async fn main() {
    let opt: Opt = Opt::from_args();
    let log_level = if opt.verbose {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };
    // All our logging goes to stderr, so our output can go to stdout
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_writer(std::io::stderr)
        .init();

    let proxy_url = match get_http_proxy_url(opt.url.scheme(), opt.proxy_url.clone()) {
        Ok(u) => u,
        Err(e) => {
            error!("Failed to obtain proxy URL: {}", e);
            std::process::exit(-1);
        }
    };
    let result = match opt.url.scheme() {
        Scheme::Http | Scheme::Https => http_request(opt.url, proxy_url, opt.req).await,
        Scheme::WebSocket | Scheme::SecureWebSocket => match opt.proxy_url {
            Some(_) => Err(Error::invalid_params(
                "proxies are only supported for use with HTTP clients at present",
            )),
            None => websocket_request(opt.url, opt.req).await,
        },
    };
    if let Err(e) = result {
        error!("Failed: {}", e);
        std::process::exit(-1);
    }
}

// Retrieve the proxy URL with precedence:
// 1. If supplied, that's the proxy URL used.
// 2. If not supplied, but environment variable HTTP_PROXY or HTTPS_PROXY are
//    supplied, then use the appropriate variable for the URL in question.
fn get_http_proxy_url(url_scheme: Scheme, proxy_url: Option<Url>) -> Result<Option<Url>> {
    match proxy_url {
        Some(u) => Ok(Some(u)),
        None => match url_scheme {
            Scheme::Http => std::env::var("HTTP_PROXY").ok(),
            Scheme::Https => std::env::var("HTTPS_PROXY")
                .ok()
                .or_else(|| std::env::var("HTTP_PROXY").ok()),
            _ => {
                if std::env::var("HTTP_PROXY").is_ok() || std::env::var("HTTPS_PROXY").is_ok() {
                    warn!(
                        "Ignoring HTTP proxy environment variables for non-HTTP client connection"
                    );
                }
                None
            }
        }
        .map(|u| u.parse())
        .transpose(),
    }
}

async fn http_request(url: Url, proxy_url: Option<Url>, req: Request) -> Result<()> {
    let client = match proxy_url {
        Some(proxy_url) => {
            info!(
                "Using HTTP client with proxy {} to submit request to {}",
                proxy_url, url
            );
            HttpClient::new_with_proxy(url, proxy_url)
        }
        None => {
            info!("Using HTTP client to submit request to: {}", url);
            HttpClient::new(url)
        }
    }?;

    match req {
        Request::ClientRequest(r) => client_request(&client, r).await,
        _ => Err(Error::invalid_params("HTTP/S clients do not support subscription capabilities (please use the WebSocket client instead)"))
    }
}

async fn websocket_request(url: Url, req: Request) -> Result<()> {
    info!("Using WebSocket client to submit request to: {}", url);
    let (client, driver) = WebSocketClient::new(url).await?;
    let driver_hdl = tokio::spawn(async move { driver.run().await });

    let result = match req {
        Request::ClientRequest(r) => client_request(&client, r).await,
        Request::Subscribe {
            query,
            max_events,
            max_time,
        } => subscription_client_request(&client, query, max_events, max_time).await,
    };

    client.close()?;
    driver_hdl
        .await
        .map_err(|e| Error::client_internal_error(e.to_string()))??;
    result
}

async fn client_request<C>(client: &C, req: ClientRequest) -> Result<()>
where
    C: Client + Sync,
{
    let result = match req {
        ClientRequest::AbciInfo => serde_json::to_string_pretty(&client.abci_info().await?)?,
        ClientRequest::AbciQuery {
            path,
            data,
            height,
            prove,
        } => serde_json::to_string_pretty(
            &client
                .abci_query(
                    path.map(|s| Path::from_str(&s)).transpose()?,
                    data,
                    height.map(Into::into),
                    prove,
                )
                .await?,
        )?,
        ClientRequest::Block { height } => {
            serde_json::to_string_pretty(&client.block(height).await?)?
        }
        ClientRequest::Blockchain { min, max } => {
            serde_json::to_string_pretty(&client.blockchain(min, max).await?)?
        }
        ClientRequest::BlockResults { height } => {
            serde_json::to_string_pretty(&client.block_results(height).await?)?
        }
        ClientRequest::BroadcastTxAsync { tx } => serde_json::to_string_pretty(
            &client
                .broadcast_tx_async(Transaction::from(tx.into_bytes()))
                .await?,
        )?,
        ClientRequest::BroadcastTxCommit { tx } => serde_json::to_string_pretty(
            &client
                .broadcast_tx_commit(Transaction::from(tx.into_bytes()))
                .await?,
        )?,
        ClientRequest::BroadcastTxSync { tx } => serde_json::to_string_pretty(
            &client
                .broadcast_tx_sync(Transaction::from(tx.into_bytes()))
                .await?,
        )?,
        ClientRequest::Commit { height } => {
            serde_json::to_string_pretty(&client.commit(height).await?)?
        }
        ClientRequest::LatestBlock => serde_json::to_string_pretty(&client.latest_block().await?)?,
        ClientRequest::LatestBlockResults => {
            serde_json::to_string_pretty(&client.latest_block_results().await?)?
        }
        ClientRequest::LatestCommit => {
            serde_json::to_string_pretty(&client.latest_commit().await?)?
        }
        ClientRequest::ConsensusState => {
            serde_json::to_string_pretty(&client.consensus_state().await?)?
        }
        ClientRequest::Genesis => serde_json::to_string_pretty(&client.genesis().await?)?,
        ClientRequest::Health => serde_json::to_string_pretty(&client.health().await?)?,
        ClientRequest::NetInfo => serde_json::to_string_pretty(&client.net_info().await?)?,
        ClientRequest::Status => serde_json::to_string_pretty(&client.status().await?)?,
        ClientRequest::TxSearch {
            query,
            page,
            per_page,
            order,
            prove,
        } => serde_json::to_string_pretty(
            &client
                .tx_search(query, prove, page, per_page, order)
                .await?,
        )?,
        ClientRequest::Validators {
            height,
            all,
            page,
            per_page,
        } => {
            let paging = if all {
                Paging::All
            } else {
                match page {
                    Some(page) => match per_page {
                        Some(per_page) => Paging::Specific {
                            page_number: page.try_into()?,
                            per_page: per_page.try_into()?,
                        },
                        None => Paging::Default,
                    },
                    None => Paging::Default,
                }
            };
            serde_json::to_string_pretty(&client.validators(height, paging).await?)?
        }
    };
    println!("{}", result);
    Ok(())
}

async fn subscription_client_request<C>(
    client: &C,
    query: Query,
    max_events: Option<u32>,
    max_time: Option<u32>,
) -> Result<()>
where
    C: SubscriptionClient,
{
    info!("Creating subscription for query: {}", query);
    let subs = client.subscribe(query).await?;
    match max_time {
        Some(secs) => recv_events_with_timeout(subs, max_events, secs).await,
        None => recv_events(subs, max_events).await,
    }
}

async fn recv_events_with_timeout(
    mut subs: Subscription,
    max_events: Option<u32>,
    timeout_secs: u32,
) -> Result<()> {
    let timeout = tokio::time::sleep(Duration::from_secs(timeout_secs as u64));
    let mut event_count = 0u64;
    tokio::pin!(timeout);
    loop {
        tokio::select! {
            result_opt = subs.next() => {
                let result = match result_opt {
                    Some(r) => r,
                    None => {
                        info!("The server terminated the subscription");
                        return Ok(());
                    }
                };
                let event = result?;
                println!("{}", serde_json::to_string_pretty(&event)?);
                event_count += 1;
                if let Some(me) = max_events {
                    if event_count >= (me as u64) {
                        info!("Reached maximum number of events: {}", me);
                        return Ok(());
                    }
                }
            }
            _ = &mut timeout => {
                info!("Reached event receive timeout of {} seconds", timeout_secs);
                return Ok(())
            }
        }
    }
}

async fn recv_events(mut subs: Subscription, max_events: Option<u32>) -> Result<()> {
    let mut event_count = 0u64;
    while let Some(result) = subs.next().await {
        let event = result?;
        println!("{}", serde_json::to_string_pretty(&event)?);
        event_count += 1;
        if let Some(me) = max_events {
            if event_count >= (me as u64) {
                info!("Reached maximum number of events: {}", me);
                return Ok(());
            }
        }
    }
    info!("The server terminated the subscription");
    Ok(())
}
