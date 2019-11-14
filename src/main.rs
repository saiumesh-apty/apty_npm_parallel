use futures::executor::block_on;
mod cmd;
use cmd::command::run_command;
use futures::future::join;
use std::error::Error;
use std::thread;
use crate::cmd::command::run_command_player;

async fn run_parr() {
//    let admin = run_command("cmd", vec!["/c".to_string(), "npm".to_string(), "install".to_string(), "http-server".to_string(), "-g".to_string()]);
//    let something = run_command("cmd", vec!["/c".to_string(), "nspm".to_string(), "-v".to_string()]);
//    join(admin, something).await;
    println!("done!!");
}

fn main() {
    let c1 = thread::spawn(|| {
        println!("started");
        block_on(run_command("cmd", vec!["/c".to_string(), "npm".to_string(), "install".to_string()]));
    });
    let c2 = thread::spawn(|| {
        println!("started22");
        block_on(run_command_player("cmd", vec!["/c".to_string(), "npm".to_string(), "install".to_string()]));
    });
    c1.join();
    c2.join();
    let command = block_on(run_parr());
}
