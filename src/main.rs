#![warn(missing_debug_implementations, rust_2018_idioms)]
#[macro_use]
extern crate clap;
#[macro_use]
extern crate anyhow;

use anyhow::Result;
use clap::Arg;
use std::io::{self, Read};
use std::process::Command;

#[derive(Debug)]
struct InputCfg {
    username: Option<String>,
    pass_name: Option<String>,
    path: Option<String>,
    host: Option<String>,
}

fn parse_input(input_cfg: &mut InputCfg) -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let lines: Vec<&str> = buffer.lines().collect();
    for line in &lines {
        let elts: Vec<&str> = line.split("=").collect();
        if elts.len() < 2 {
            break;
        }
        let k = elts[0];
        let v = elts[1..].join("");
        match k {
            "username" => {
                if input_cfg.username.is_none() {
                    input_cfg.username = Some(v)
                }
            }
            "host" => input_cfg.host = Some(v),
            "path" => input_cfg.path = Some(v),
            _ => (),
        }
    }
    Ok(())
}

fn get_pass_name(input_cfg: &InputCfg) -> Result<String> {
    Ok(format!(
        "{}/{}",
        input_cfg.host.clone().ok_or(anyhow!("host is missing"))?,
        input_cfg
            .username
            .clone()
            .ok_or(anyhow!("username is missing"))?
    ))
}

fn show_pass(pass_name: &str) -> Result<()> {
    let pass_cmd_res = Command::new("pass").arg("show").arg(pass_name).output()?;
    if pass_cmd_res.status.success() {
        println!("password={}", std::str::from_utf8(&pass_cmd_res.stdout)?);
        Ok(())
    } else {
        Err(anyhow!(
            "pass command exited with status=[{}],err=[{}]",
            pass_cmd_res.status.code().unwrap(),
            std::str::from_utf8(&pass_cmd_res.stderr)?
        ))
    }
}

fn main() -> Result<()> {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("username")
                .long("username")
                .short("u")
                .value_name("username")
                .takes_value(true)
                .help("override the user to find the password"),
        )
        .arg(
            Arg::with_name("pass_name")
                .long("pass_name")
                .short("p")
                .value_name("pass_name")
                .takes_value(true)
                .help("set the pass_name to the pass entry"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("git credentials operation")
                .required(true)
                .index(1),
        )
        .get_matches();
    match matches.value_of("INPUT") {
        Some("get") => Ok(()),
        Some(operation) => Err(anyhow!("Unsupported operation [{}]", operation)),
        _ => Err(anyhow!("Missing operation")),
    }?;
    let username = matches.value_of("username").map(|u| String::from(u));
    let pass_name_opt = matches.value_of("pass_name").map(|p| String::from(p));
    let pass_name: String;
    if let Some(p) = pass_name_opt {
        pass_name = p;
    } else {
        let mut cfg = InputCfg {
            username,
            pass_name: None,
            path: None,
            host: None,
        };
        parse_input(&mut cfg)?;
        pass_name = get_pass_name(&cfg)?;
    }
    show_pass(&pass_name)?;
    Ok(())
}
