use std::collections::HashMap;

use mpd::Client;
use mpd::song::Song;
use mpd::status::{ Status, State };
use mpd::search::{ Query, Term, Window };

use lazy_static::lazy_static;
 
use crate::lexer::*;

fn search(conn: &mut Client, query_text: &str) -> Vec<Song> {
    let mut q = Query::new();
    let p = q.and(Term::File, query_text);
    let results = conn.search(&p, (1, 10)).unwrap();
    return results;
}

fn play(conn: &mut Client, args: Option<String>) {
    match args {
        Some(a) => {
            let song_path = a;
            conn.push(song_path);
        },
        None => {
            conn.toggle_pause().unwrap();
        }
    }
}

fn list(conn: &mut Client) {
    let queue = conn.queue();
    match queue {
        Ok(songs) => {
            for song in songs.iter() {
                let artist = match song.tags.get("Artist") {
                    Some(s) => s,
                    None => "N/A"
                };
                let album = match song.tags.get("Album") {
                    Some(s) => s,
                    None => "N/A"
                };
                let title = match song.title {
                    Some(ref x) => x.clone(),
                    None => "N/A".into()
                };
                println!("{} // {} // {}", artist, album, title);
            }
        }
        Err(_) => {
            println!("Unable to print songs in queue.");
        }
    }
}

// lazy_static! {
//     static ref FUNCTIONS: HashMap<&'static str, Fn>;
// }

// fn get_functions(&mut Client, Option<Vec<String>>) -> Result<()> {
//     let mut f = HashMap::new();
//     f.insert("play"  , play);
//     f.insert("pause" , pause);
//     f.insert("stop"  , stop);
//     f.insert("add"   , add);
// }
