use std::path::PathBuf;
use std::net::{ IpAddr, Ipv4Addr, SocketAddr };
use std::fs::{ OpenOptions };

use mpd::Client;
use mpd::status::{ Status, State };

use directories::BaseDirs;

use rustyline::config::{ Builder, Config, CompletionType, EditMode };
use rustyline::Editor;
use rustyline::error::ReadlineError;

use serde::Deserialize;

use lazy_static::lazy_static;

#[derive(Deserialize)]
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

    let line_config: Config = Builder::new()
        .max_history_size(100)
        .history_ignore_dups(true)
        .history_ignore_space(true)
        .completion_type(CompletionType::Circular)
        .edit_mode(EditMode::Emacs)
        .build();
    let mut rl = Editor::<()>::with_config(line_config);

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

fn handler(line: String, conn: &mut Client) {
    let parts: Vec<&str> = (&line).split(' ').collect();
    match *parts.first().unwrap() {
        "play" => {
            conn.toggle_pause().unwrap();
            println!("{}", now_playing(conn).unwrap());
        }
        "pause" => {
            conn.pause(true).unwrap();
            println!("{}", now_playing(conn).unwrap());
        },
        "stop" => conn.stop().unwrap(),
        _ => {}
    }
}

fn now_playing(conn: &mut Client) -> Option<String> {
    let song_wrapped = conn.currentsong();
    match song_wrapped {
        Ok(song) => {
            if let Some(s) = song {
                let title  = s.title.unwrap_or("N/A".into());
                let artist = s.tags.get("Artist");
                let album  = s.tags.get("Album");
                return Some(format!("{} // {} // {}",
                                    artist.unwrap(), album.unwrap(), title));
            } else { None }
        }
        _ => None
    }
}

fn read_config() -> ClientConfig {
    let mut mush_config_path = PathBuf::new();

    if let Some(mush_dirs) = BaseDirs::new() {
        mush_config_path.push(mush_dirs.config_dir());
        mush_config_path.push("mush");
        mush_config_path.set_extension("toml");
    } else {
        panic!("No home directory found!");
    }

    let mut configuration = ClientConfig::new();

    let config_file_contents = std::fs::read_to_string(mush_config_path);
    match config_file_contents.as_ref()  {
        Ok(c) => configuration = toml::from_str(c).unwrap(),
        _ => {}
    };
    return configuration;
}
