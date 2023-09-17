use warp::Filter;
use leveldb::database::bytes;


fn start_server(port: i32, db_path: String, volumes: Vec<String>, replicas: usize, subvolumes: usize) {
    let put = warp::put()
        .and(warp::path::param::<String>())
        .and(warp::body::bytes())
        .map(|key: String, body: bytes::Bytes| {
            // TODO: Implement
            warp::reply::with_status("PUT placeholder", warp::http::StatusCode::CREATED)
        });

    let get = warp::get()
        .and(warp::path::param::<String>())
        .map(|key: String| {
            // TODO: Implement
            warp::reply::with_status("GET placeholder", warp::http::StatusCode::OK)
        });

    let delete = warp::delete()
        .and(warp::path::param::<String>())
        .map(|key: String| {
            // TODO: Implement
            warp::reply::with_status("DELETE placeholder", warp::http::StatusCode::NO_CONTENT)
        });

    let head = warp::head()
        .and(warp::path::param::<String>())
        .map(|key: String| {
            // TODO: Implement
            warp::reply::with_status("HEAD placeholder", warp::http::StatusCode::OK)
        });

    let post = warp::post()
        .and(warp::path::param::<String>())
        .and(warp::body::bytes())
        .map(|key: String, body: bytes::Bytes| {
            // TODO: Implement
            warp::reply::with_status("POST placeholder", warp::http::StatusCode::CREATED)
        });

    let unlink = warp::method("UNLINK")
        .and(warp::path::param::<String>())
        .map(|key: String| {
            // TODO: Implement
            warp::reply::with_status("UNLINK placeholder", warp::http::StatusCode::OK)
        });

    let rebalance = warp::method("REBALANCE")
        .and(warp::path::param::<String>())
        .map(|key: String| {
            // TODO: Implement
            warp::reply::with_status("REBALANCE placeholder", warp::http::StatusCode::OK)
        });

    let routes = put.or(get).or(delete).or(head).or(post).or(unlink).or(rebalance);
    warp::serve(routes).run(([127, 0, 0, 1], port as u16));
}