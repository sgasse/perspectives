#[tokio::main]
async fn main() {
    let route = warp::fs::dir("www");

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
