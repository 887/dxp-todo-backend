
use std::{thread};
use std::sync::{Arc};

use hot_lib_reloader::BlockReload;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::RwLock;
use tokio::{sync::mpsc, task::spawn_blocking};

//tokio hot reload example
//https://github.com/rksm/hot-lib-reloader-rs/blob/master/examples/reload-events/src/main.rs

#[hot_lib_reloader::hot_module(dylib = "lib")]
mod hot_lib {
    use poem::{Route};
    use poem::{listener::TcpListener, Server};
    use std::convert::Infallible;

    // pub use lib::*;

    hot_functions_from_file!("lib/src/lib.rs");

    // expose a type to subscribe to lib load events
    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Use RUST_LOG=hot_lib_reloader=trace to see all related logs
    env_logger::init();

    #[cfg(all(feature = "path-info"))]
    println!("working directory {}", get_current_working_dir());
    #[cfg(all(feature = "path-info"))]
    println!("lib path {}", get_lib_path());

    //this channel is for lib reloads
    let (tx_lib_reloaded, mut rx_lib_reloaded) = mpsc::channel(1);

    //this ensures this will only be dropped when the main runtime is shut down
    tokio::task::spawn(async move {
        loop {
            wait_for_reload(tx_lib_reloaded.clone()).await;
        }
    });

    loop {
        //creating these channels in this loop, so only the server from this loop will be shut down

        //this channel is to shut down the server 
        let (tx_shutdown_server, rx_shutdown_server) = mpsc::channel(1);

        //this channel is to wait until the server is shut down before the reload
        let (tx_sever_was_shutdown, mut rx_server_was_shutdown) = mpsc::channel(1);

        let server_running = Arc::new(RwLock::new(false));
        let server_running_check = server_running.clone();

        let wait = async move {
            println!("trying again in 3s");
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        };

        if let Err(load_err) = hot_lib::load_env() {
            println!("hot_lib::load_env: {}", load_err);
            wait.await;
            continue;
        }

        println!("-----------------------------------");

        //using the runtime here causes thread panics, always create new threads 
        //https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
        // let migration_result = match tokio::task::spawn_blocking(|| {
        let migration_result = match thread::spawn(|| {
            run_migration()
        }).join() {
            Ok(res) => res,
            Err(_err) => {
            println!("run migration thread panicked");
            wait.await;
            continue;
        }
        };

        if let Err(load_err) = migration_result {
            println!("migration failed: {}", load_err);
            wait.await;
            continue;
        }

        let run_sever_future = async move {
            *server_running.write().await = true;

            match thread::spawn(|| {
                run_server(rx_shutdown_server)
            }).join() {
                Ok(_) => {
                    match (&tx_sever_was_shutdown).send(()).await {
                        Ok(_) => {
                            println!("server_was_shutdown signal sent");
                        }
                        Err(e) => {
                            println!("error sending server_was_shutdown signal: {:?}", e);
                        }
                    } 
                },
                Err(_err) => {
                    *server_running.write().await = false;
                    println!("run migration thread panicked");
                    wait.await;
                    return;
                }
            };

            println!("run_server finished")
        };

        let main_loop = tokio::task::spawn(run_sever_future);

        tokio::select! {
            // This simulates the normal main loop behavior...
            _ = main_loop => {
            }

            // when we receive a about-to-reload token then the reload is
            // blocked while the token is still in scope. This gives us the
            // control over how long the reload should wait.
            Some(block_reload_token) = rx_lib_reloaded.recv() => {

                //signal sever to shutdown
                if *server_running_check.read().await {
                    println!("send shutdown to server!");
                    match (&tx_shutdown_server).send(()).await {
                        Ok(_) => {
                            if *server_running_check.read().await {
                                match rx_server_was_shutdown.recv().await {
                                    Some(_) => {
                                        println!("received server_was_shutdown signal");
                                    }
                                    None => {
                                        println!("server_was_shutdown listening channel closed");
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("error sending shutdown signal: {}", e);
                        }
                    }
                }

                do_reload(block_reload_token).await;
            }
        }    
    }
}

async fn wait_for_reload(tx_lib_reloaded: Sender<BlockReload>) -> () {
    let block_reload = spawn_blocking(|| hot_lib::subscribe().wait_for_about_to_reload())
        .await
        .expect("get token");
    tx_lib_reloaded.send(block_reload).await.expect("send token");
}

fn run_migration() -> Result<(), anyhow::Error> {
    hot_lib::run_migration()
}

fn run_server(rx_shutdown_server: Receiver<()>) -> Result<(), anyhow::Error> {
    hot_lib::run_server(rx_shutdown_server)
}

async fn do_reload(block_reload_token: BlockReload) {

    println!("...now we are ready for reloading...");
    // Now drop the token, allow the reload
    drop(block_reload_token); // token drop causes reload to continue

    println!("trying reload");
    // Now we wait for the lib to be reloaded...
    spawn_blocking(|| hot_lib::subscribe().wait_for_reload())
        .await
        .expect("wait for reload");
    println!("...now we have the new library version loaded");

    // now main loop with tokio::select! continues and restarts all the primary futures
}

#[cfg(all(feature = "path-info"))]
fn get_current_working_dir() -> String {
    let res = std::env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string(),
    }
}

#[cfg(all(feature = "path-info"))]
fn get_lib_path() -> String {
    let res = std::env::var("LD_LIBRARY_PATH");
    match res {
        Ok(path) => path.to_string(),
        Err(_) => "FAILED".to_string(),
    }
}
