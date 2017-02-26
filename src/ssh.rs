use std::io::Read;
use std::net::TcpStream;
use std::path::Path;
use ansi_term::Colour;
use ssh2::{Error, Session};
use yaml_rust::yaml;


pub fn execute(dsn: &str, key: &str, commands: &Vec<yaml::Yaml>) {
    let user_host = dsn.split("@").collect::<Vec<&str>>();
    let username = user_host[0];
    let host = user_host[1];

    let tcp = match TcpStream::connect(host) {
        Ok(tcp) => tcp,
        Err(err) => {
            println!("{:#?}", err);
            return;
        }
    };

    let mut session = Session::new().unwrap();
    match session.handshake(&tcp) {
        Ok(_) => {}
        Err(err) => {
            println!("{:#?}", err);
            return;
        }
    }

    session = match auth_with_keys(session, username, key) {
        Ok(sess) => sess,
        Err(err) => {
            println!("{:#?}", err);
            return;
        }
    };

    assert!(session.authenticated());

    println!("      {} {}", Colour::Green.paint("ssh"), Colour::Blue.paint("ok"));
    println!(" {}", Colour::Green.paint("commands"));
    for command in commands {
        println!("          {}", Colour::Blue.paint(command.as_str().unwrap()));
        let mut channel = session.channel_session().unwrap();
        channel.exec(command.as_str().unwrap()).unwrap();
        {
            let bytes = channel.by_ref().bytes();
            let mut line = String::from("");
            for byte in bytes {
                line.push(byte.unwrap() as char);
                if line.ends_with("\n") {
                    print!("              {}", line);
                    line = String::from("");
                }
            }
        }
        println!();
        let exit_status = channel.exit_status().unwrap();
        if exit_status > 0 {
            println!("    Exited with status {}", exit_status);
            break;
        }
    }
}

fn auth_with_keys(sess: Session, username: &str, key: &str) -> Result<Session, Error> {
    let mut filename = key.to_string();
    if !key.to_lowercase().ends_with(".pub") {
        filename = filename + ".pub";
    }
    let key_public = &Path::new(&filename);
    let key_private = &Path::new(&key);

    match sess.userauth_pubkey_file(username, Some(key_public), key_private, Some(username)) {
        Ok(_) => Ok(sess),
        Err(err) => Err(err),
    }
}

#[allow(dead_code)]
fn auth_with_agent(sess: Session, username: &str) -> Result<Session, Error> {
    match sess.userauth_agent(username) {
        Ok(_) => Ok(sess),
        Err(err) => Err(err),
    }
}

#[allow(dead_code)]
fn auth_with_password(sess: Session, username: &str, password: &str) -> Result<Session, Error> {
    match sess.userauth_password(username, password) {
        Ok(_) => Ok(sess),
        Err(err) => Err(err),
    }
}
