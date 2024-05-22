use std::env;

use hot_lib_reloader::BlockReload;
use tokio::spawn;
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

    println!("working directory {}", get_current_working_dir());
    println!("lib path {}", get_lib_path());

    //this channel is for lib reloads
    let (tx_lib_reloaded, mut rx_lib_reloaded) = mpsc::channel(1);

    tokio::task::spawn(async move {
        loop {
            let block_reload = spawn_blocking(|| hot_lib::subscribe().wait_for_about_to_reload())
                .await
                .expect("get token");
            tx_lib_reloaded.send(block_reload).await.expect("send token");
        }
    });

    loop {
        //this channel is to shut down the server
        let (tx_shutdown_server, mut rx_shutdown_server) = mpsc::channel(1);

        //this channel is to wait until the server is shut down before the reload
        let (tx_sever_was_shutdown, mut rx_server_was_shutdown) = mpsc::channel(1);

        let tx_sever_was_shutdown_unexpected = tx_sever_was_shutdown.clone();
        let main_loop_future = async move {
            println!("-----------------------------------");

            let wait = async move {
                println!("trying again in 3s");
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                println!("sending server_was_shutdown signal");
                match (tx_sever_was_shutdown_unexpected).send(()).await {
                    Ok(_) => {
                        println!("server_was_shutdown signal sent");
                    }
                    Err(e) => {
                        println!("error sending server_was_shutdown signal: {:?}", e);
                    }
                }
            };

            println!("hot_lib::get_assembled_server");
            let server = match hot_lib::get_assembled_server() {
                Ok(server) => server,
                Err(err) => {
                    println!("hot_lib::get_assembled_server failed: {}", err);
                    wait.await;
                    return;
                }
            };

            let endpoints = match hot_lib::get_endpoints() {
                Ok(endpoints) => endpoints,
                Err(e) => {
                    println!("error in hot_lib::get_endpoints: {:?}", e);
                    wait.await;
                    return;
                }
            };

            let run_server_future = async move {
                println!("running server now");
                let server_result = server.run_with_graceful_shutdown(endpoints, async move {
                    match (rx_shutdown_server).recv().await {
                        Some(_) => {
                            println!("received shutdown_server signal, time to shut down");
                        }
                        None => {
                            println!("shutdown_server listening channel closed");
                        }
                    }
                }, None).await;
                match server_result {
                    Ok(_) => {
                        println!("server has been shut down successfully");
                    }
                    Err(e) => {
                        println!("server shut down with error: {:?}", e);
                    }
                }

                match (tx_sever_was_shutdown).send(()).await {
                    Ok(_) => {
                        println!("server_was_shutdown signal sent");
                    }
                    Err(e) => {
                        println!("error sending server_was_shutdown signal: {:?}", e);
                    }
                } 
            };

            //https://users.rust-lang.org/t/there-is-no-reactor-running-must-be-called-from-the-context-of-a-tokio-1-x-runtime/75393/3
            let _ = spawn(run_server_future).await;
        };

        tokio::select! {
            // This simulates the normal main loop behavior...
            _ = main_loop_future => {
            }

            // when we receive a about-to-reload token then the reload is
            // blocked while the token is still in scope. This gives us the
            // control over how long the reload should wait.
            Some(block_reload_token) = rx_lib_reloaded.recv() => {
                //signal sever to shutdown
                println!("send shutdown to server!");
                match (&tx_shutdown_server).send(()).await {
                    Ok(_) => {
                        println!("shutdown signal was sent");
                    }
                    Err(e) => {
                        println!("error sending shutdown signal: {}", e);
                    }
                }

                match rx_server_was_shutdown.recv().await {
                    Some(_) => {
                        println!("received server_was_shutdown signal");
                    }
                    None => {
                        println!("server_was_shutdown listening channel closed");
                    }
                }

                do_reload(block_reload_token).await;
            }
        }    
    } 
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

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string(),
    }
}

fn get_lib_path() -> String {
    let res = env::var("LD_LIBRARY_PATH");
    match res {
        Ok(path) => path.to_string(),
        Err(_) => "FAILED".to_string(),
    }
}
