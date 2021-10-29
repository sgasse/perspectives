use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Self-Serving WASM Project")]
struct Opts {
    /// Port on which to serve.
    #[structopt(short, long, default_value = "3050")]
    port: u16,

    /// Path of the directory to serve, relative or absolute.
    #[structopt(short, long, default_value = "www")]
    serve_dir: String,

    /// Bind to all IP addresses
    #[structopt(short, long)]
    bindall: bool,
}

#[tokio::main]
async fn main() {
    let opts = Opts::from_args();
    let route = warp::fs::dir(opts.serve_dir);

    let mut ip_arr = [127, 0, 0, 1];
    if opts.bindall {
        println!("Binding to all IP addresses");
        ip_arr = [0, 0, 0, 0];
    }

    warp::serve(route).run((ip_arr, opts.port)).await;
}
