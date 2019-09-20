use std::collections::HashMap;

use mpd::Client;
use mpd::status::{ Status, State };
use mpd::search::{ Query, Term, Window };
 
use crate::lexer::*;

fn search(&str: query_text) -> Vec<Song> {
    let mut q = Query::new();
    let p = q.and(Term::File, query_text);
    let results = conn.search(&p, (1, 10)).unwrap();
    let songs = results.iter()
        .collect();
    return songs;
}

fn play(&mut conn: Client, args: Option<Vec<String>>) -> Result<()> {

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
                let title = song.title.map_or_else(|| "N/A", |x| &&x);
                println!("{} // {} // {}", artist, album, title);
            }
        }
        Err(_) => {
            println!("Unable to print songs in queue.");
        }
    }
}

fn parse(cmd: lexer::Command) -> {
    if let Some(c) = cmd.command {

    }
}
