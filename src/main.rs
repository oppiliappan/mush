use std::path::PathBuf;
use std::net::{ IpAddr, Ipv4Addr, SocketAddr };
use std::fs;

use mpd::Client;
use mpd::status::{ Status, State };
use mpd::search::{ Query, Term, Window };

use directories::BaseDirs;
use serde::Deserialize;
use lazy_static::lazy_static;

mod readline;
use crate::readline::*;
use rustyline::error::ReadlineError;

mod lexer;
use crate::lexer::*;

mod parser;
use crate::parser::*;

#[derive(Deserialize, Debug)]
struct ClientConfig {
    ip: Ipv4Addr,
    port: Option<u16>,
}

impl ClientConfig {
    fn new() -> ClientConfig {
        return ClientConfig {
            ip: Ipv4Addr::new(127, 0, 0, 1),
            port: Some(6600)
        };
    }
    fn socket_addr(&self) -> SocketAddr {
        return SocketAddr::new(IpAddr::V4(self.ip), self.port.unwrap_or(6600));
    }
}

lazy_static! {
    static ref CONFIGURATION: ClientConfig = read_config();
}

fn main() {
    let socket = CONFIGURATION.socket_addr();
    let mut conn = Client::connect(socket).unwrap();
    let mut rl = create_readline();

    loop {
        let line_read = rl.readline("> ");
        match line_read {
            Ok(line) => {
                handler(line, &mut conn);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}

// TODO: commands to handle
// play
// pause
// stop
// fade
// next
// prev
// repeat
// now playing
//
// Queue:
// add to queue
//

fn handler(line: String, conn: &mut Client) {
    let token: lexer::Command = lexer::lex(&line);
    println!("{:#?}", token);
}

fn now_playing(conn: &mut Client) -> Option<String> {
    let song_wrapped = conn.currentsong();
    match song_wrapped {
        Ok(song) => {
            if let Some(s) = song {
                // let title  = s.title.unwrap_or("N/A".into());
                // let artist = s.tags.get("Artist");
                // let album  = s.tags.get("Album");
                // return Some(format!("{} // {} // {}",
                //                     artist.unwrap(), album.unwrap(), title));
                return Some(format!("{:#?}", s));
            } else { None }
        }
        _ => None
    }
}

fn list_all(conn: &mut Client, query_text: &str) -> Vec<String> {
    let mut q = Query::new();
    let p = q.and(Term::File, query_text);
    let results = conn.search(&p, (1, 10)).unwrap();
    let songs: Vec<String> = results.iter()
        .map(|x| format!("{:?}", x))
        .collect();
    return songs;
}

fn read_config() -> ClientConfig {
    let mut mush_config_path = PathBuf::new();

    if let Some(mush_dirs) = BaseDirs::new() {
        mush_config_path.push(mush_dirs.config_dir());
        mush_config_path.push("mush");
        fs::create_dir_all(&mush_config_path).unwrap();
        mush_config_path.push("config.toml");
    } else {
        panic!("No home directory found!");
    }

    let mut configuration = ClientConfig::new();

    let config_file_contents = std::fs::read_to_string(mush_config_path);
    match config_file_contents.as_ref()  {
        Ok(c) => {
            configuration = toml::from_str(c).unwrap();
        },
        _ => {}
    };
    println!("{:?}", configuration);
    return configuration;
}
