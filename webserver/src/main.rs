use warp::Filter;

#[tokio::main]
async fn main() {

    let body = r#"<!DOCTYPE HTML>
    <HTML>
      <body>
        <a href="/dummy/">dummy</a>
      </body>
    </html>"#;

    let dummy = r#"<!DOCTYPE HTML>
    <HTML>
      <body>
        <a href="../packages/dummy-0.1.tar.gz">dummy</a>
      </body>
    </html>"#;

    let simple = warp::path!("simple")
        .and(warp::get())
        .map(move || {
            warp::reply::html(body)
        });
    
    let dummy = warp::path!("simple" / "dummy")
        .and(warp::get())
        .map(move || {
            warp::reply::html(dummy)
        });

    let dummy_tar_gz = warp::path!("simple" / "packages" / "dummy-0.1.tar.gz")
        .and(warp::fs::file("packages/dummy-0.1.tar.gz"));

    let routes = simple.or(dummy).or(dummy_tar_gz);

    warp::serve(routes)
    .run(([127, 0, 0, 1], 3030))
    .await;
}
