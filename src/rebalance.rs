use crate::lib;
use crate::app::App;
use std::time::Duration;
use std::collections::HashSet;



#[derive(Clone)]
struct RebalanceRequest {
    key: Vec<u8>,
    volumes: Vec<String>,
    kvolumes: Vec<String>,
}

async fn rebalance(app: &App, req: RebalanceRequest) -> bool {
    let kp = lib::key2path(&req.key);
    
    // find the volumes that are real
    let mut rvolumes = Vec::new();
    for rv in &req.volumes {
        let remote_test = format!("http://{}{}", rv, kp);
        match lib::remote_head(&remote_test, Duration::from_secs(60)).await {
            Ok(found) => {
                if found {
                    rvolumes.push(rv.clone());
                }
            },
            Err(err) => {
                println!("rebalance head error {:?} {}", err, remote_test);
                return false;
            }
        }
    }

    if rvolumes.is_empty() {
        println!("rebalance impossible, {} is missing!", String::from_utf8_lossy(&req.key));
        return false;
    }

    if !lib::needs_rebalance(&rvolumes, &req.kvolumes) {
        return true;
    }

    println!("rebalancing {} from {:?} to {:?}", String::from_utf8_lossy(&req.key), rvolumes, req.kvolumes);

    // find a good rvolume
    let mut ss = String::new();
    for v in &rvolumes {
        let remote_from = format!("http://{}{}", v, kp);
        match lib::remote_get(&remote_from).await {
            Ok(data) => {
                ss = data;
                break;
            },
            Err(err) => {
                println!("rebalance get error {:?} {}", err, remote_from);
            }
        }
    }
    if ss.is_empty() {
        return false;
    }

    // write to the kvolumes
    let mut rebalance_error = false;
    for v in &req.kvolumes {
        let mut needs_write = true;
        for v2 in &rvolumes {
            if v == v2 {
                needs_write = false;
                break;
            }
        }
        if needs_write {
            let remote_to = format!("http://{}{}", v, kp);
            if let Err(err) = lib::remote_put(&remote_to, ss.as_bytes().to_vec()).await {
                println!("rebalance put error {:?} {}", err, remote_to);
                rebalance_error = true;
            }
        }
    }
    if rebalance_error {
        return false;
    }

    // update db
    if !app.put_record(req.key, lib::Record {
        rvolumes: HashSet::from_iter(req.kvolumes.clone()),
        deleted: lib::Deleted::No,
        hash: String::new(),
    }) {
        println!("rebalance put db error");
        return false;
    }

    // delete from the volumes that now aren't kvolumes
    let mut delete_error = false;
    for v2 in &rvolumes {
        let mut needs_delete = true;
        for v in &req.kvolumes {
            if v == v2 {
                needs_delete = false;
                break;
            }
        }
        if needs_delete {
            let remote_del = format!("http://{}{}", v2, kp);
            if let Err(err) = lib::remote_delete(&remote_del).await {
                println!("rebalance delete error {:?} {}", err, remote_del);
                delete_error = true;
            }
        }
    }
    !delete_error
}

pub fn All(app: &App) {
    let volumes = app.volumes.clone();
    let keys = []; // TODO: add db // app.db.keys();

    // Process each key synchronously without spawning threads
    for key in keys {
        let kvolumes = lib::key2volume(key, &app.volumes, app.replicas, app.subvolumes);
        let req = RebalanceRequest {
            key: key.to_vec(),
            volumes: volumes.clone(),
            kvolumes: kvolumes.clone(),
        };
        rebalance(app, req);
    }
}


// pub fn All(app: &App) {
//     let app = Arc::new(app);
//     let volumes = app.volumes.clone();
//     let keys = []; // TODO: add db // app.db.keys();
//     let (tx, rx) = std::sync::mpsc::channel();
//     let rx = Arc::new(Mutex::new(rx));

//     // Spawn workers
//     const NUM_WORKERS: usize = 10;
//     let mut handles = vec![];
//     for _ in 0..NUM_WORKERS {
//         let app = app.clone();
//         let rx = rx.clone();
//         let handle = std::thread::spawn(move || loop {
//             let maybe_req = {
//                 let r = rx.lock().unwrap();
//                 r.try_recv()
//             };

//             match maybe_req {
//                 Ok(req) => rebalance(&app, req),
//                 Err(TryRecvError::Disconnected) => break,
//                 Err(TryRecvError::Empty) => continue,
//             };
//         });
//         handles.push(handle);
//     }

//     // Send tasks to workers
//     for key in keys {
//         let kvolumes = lib::key2volume(key, &app.volumes, app.replicas, app.subvolumes);
//         let req = RebalanceRequest {
//             key: key.to_vec(),
//             volumes: volumes.clone(),
//             kvolumes: kvolumes.clone(),
//         };
//         tx.send(req).unwrap();
//     }

//     // Close the channel and wait for all workers to finish
//     drop(tx);
//     for handle in handles {
//         handle.join().unwrap();
//     }
// }
