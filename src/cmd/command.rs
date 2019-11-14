use std::process::{ Command, Stdio };
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub async fn run_command(command: &str, args: Vec<String>) -> Result<(), Error>{
    let output = Command::new(command).args(args)
        .current_dir("admin")
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;
    println!("{}", String::from_utf8(output.stdout).unwrap());
    Ok(())
}

pub async fn run_command_player(command: &str, args: Vec<String>) -> Result<(), Error>{
    let output = Command::new(command).args(args)
        .current_dir("player3.0")
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;
    println!("{}", String::from_utf8(output.stdout).unwrap());
    Ok(())
}