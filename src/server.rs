use warp::{Buf, Filter, reply::WithStatus};


fn custom_method(method: &str) -> warp::filters::BoxedFilter<()> {
    warp::header::<String>("upgrade")
        .and_then(|method: String| async move {
            if method.to_uppercase() == method {
                Ok(())
            } else {
                Err(warp::reject::reject())
            }
        })
        .untuple_one()
        .boxed()
}

fn post(key: String, mut body: impl Buf) -> WithStatus<&'static str> {
    // TODO: Implement
    warp::reply::with_status("POST placeholder", warp::http::StatusCode::CREATED)
}
fn put(key: String, mut body: impl Buf) -> WithStatus<&'static str> {
    // TODO: Implement
    warp::reply::with_status("PUT placeholder", warp::http::StatusCode::CREATED)
}

fn start_server(port: i32, db_path: String, volumes: Vec<String>, replicas: usize, subvolumes: usize) {
    let put = warp::put()
        .and(warp::path::param::<String>())
        .and(warp::body::bytes())
        .map(put);

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
        .map(post);

    let unlink = warp::any().and(custom_method("UNLINK"))
        .and(warp::path::param::<String>())
        .map(|key: String| {
            // TODO: Implement
            warp::reply::with_status("UNLINK placeholder", warp::http::StatusCode::OK)
        });

    let rebalance = warp::any().and(custom_method("REBALANCE"))
        .and(warp::path::param::<String>())
        .map(|key: String| {
            // TODO: Implement
            warp::reply::with_status("REBALANCE placeholder", warp::http::StatusCode::OK)
        });

    let routes = put.or(get).or(delete).or(head).or(post).or(unlink).or(rebalance);
    warp::serve(routes).run(([127, 0, 0, 1], port as u16));
}