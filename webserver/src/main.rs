use warp::{Filter, Rejection};

mod handler;

type WebResult<T> = std::result::Result<T, Rejection>;

pub struct Project {
    pub name: String,
    pub url: String,
}

pub struct PackageVersion {
    pub name: String,
    pub download_url: String,
    pub package_name: String,
}

#[tokio::main]
async fn main() {
    let simple = warp::path!("simple")
        .and(warp::get())
        .and_then(handler::project_list_handler);

    let dummy = warp::path!("simple" / "dummy")
        .and(warp::get())
        .and_then(handler::version_list_handler);

    let dummy_tar_gz_01 = warp::path!("simple" / "packages" / "dummy-0.1.tar.gz")
        .and(warp::fs::file("packages/dummy-0.1.tar.gz"));

    let dummy_tar_gz_02 = warp::path!("simple" / "packages" / "dummy-0.2.tar.gz")
        .and(warp::fs::file("packages/dummy-0.2.tar.gz"));

    let routes = simple.or(dummy).or(dummy_tar_gz_01).or(dummy_tar_gz_02);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
