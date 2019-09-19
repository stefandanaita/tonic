use structopt::StructOpt;
use tonic::Server;
use tonic_interop::server;
// TODO: move GrpcService out of client since it can be used for the
// server too.
use tonic::client::GrpcService;

#[derive(StructOpt)]
struct Opts {
    #[structopt(long)]
    use_tls: bool,
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tonic_interop::trace_init();

    let matches = Opts::from_args();

    let addr = "127.0.0.1:10000".parse().unwrap();

    let test_service = server::create();

    let mut builder = Server::builder();

    if matches.use_tls {
        let ca = tokio::fs::read("tonic-interop/data/server1.pem").await?;
        let key = tokio::fs::read("tonic-interop/data/server1.key").await?;
        builder.tls(ca, key);
    }

    builder.interceptor_fn(|svc, req| {
        println!("INBOUND REQUEST={:?}", req);
        let call = svc.call(req);
        async move {
            let res = call.await?;
            println!("OUTBOUND RESPONSE={:?}", res);
            Ok(res)
        }
    });

    builder.serve(addr, test_service).await?;

    Ok(())
}
