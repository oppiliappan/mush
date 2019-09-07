use mpd::Client;
use mpd::status::{ Status, State };

use rustyline::config::{ Builder, Config, CompletionType, EditMode };
use rustyline::Editor;
use rustyline::error::ReadlineError;

fn main() {
    let mut conn = Client::connect("127.0.0.1:6600").unwrap();

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
