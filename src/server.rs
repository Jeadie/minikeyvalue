use warp::{Buf, Filter, reply::WithStatus};


fn custom_method(expected_method: &'static str) -> warp::filters::BoxedFilter<()> {
    warp::header::<String>("upgrade")
        .and_then(move |method: String| async move {
            if method.to_uppercase() == expected_method {
                Ok(())
            } else {
                Err(warp::reject::reject())
            }
        })
        .untuple_one()
        .boxed()
}

fn post(_key: String, _body: impl Buf) -> WithStatus<&'static str> {
    // TODO: Implement
    warp::reply::with_status("POST placeholder", warp::http::StatusCode::CREATED)
}
fn put(_key: String, _body: impl Buf) -> WithStatus<&'static str> {
    // TODO: Implement
    warp::reply::with_status("PUT placeholder", warp::http::StatusCode::CREATED)
}

fn start_server(port: i32, _db_path: String, _volumes: Vec<String>, _replicas: usize, _subvolumes: usize) {
    let put = warp::put()
        .and(warp::path::param::<String>())
        .and(warp::body::bytes())
        .map(put);

    let get = warp::get()
        .and(warp::path::param::<String>())
        .map(|_key: String| {
            // TODO: Implement
            warp::reply::with_status("GET placeholder", warp::http::StatusCode::OK)
        });

    let delete = warp::delete()
        .and(warp::path::param::<String>())
        .map(|_key: String| {
            // TODO: Implement
            warp::reply::with_status("DELETE placeholder", warp::http::StatusCode::NO_CONTENT)
        });

    let head = warp::head()
        .and(warp::path::param::<String>())
        .map(|_key: String| {
            // TODO: Implement
            warp::reply::with_status("HEAD placeholder", warp::http::StatusCode::OK)
        });

    let post = warp::post()
        .and(warp::path::param::<String>())
        .and(warp::body::bytes())
        .map(post);

    let unlink = warp::any().and(custom_method("UNLINK"))
        .and(warp::path::param::<String>())
        .map(|_key: String| {
            // TODO: Implement
            warp::reply::with_status("UNLINK placeholder", warp::http::StatusCode::OK)
        });

    let rebalance = warp::any().and(custom_method("REBALANCE"))
        .and(warp::path::param::<String>())
        .map(|_key: String| {
            // TODO: Implement
            warp::reply::with_status("REBALANCE placeholder", warp::http::StatusCode::OK)
        });

    let routes = put.or(get).or(delete).or(head).or(post).or(unlink).or(rebalance);
    warp::serve(routes).run(([127, 0, 0, 1], port as u16));
}