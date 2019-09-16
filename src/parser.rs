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

lazy_static! {
    static ref FUNCTIONS: HashMap<&'static str,
    fn(&mut Client, Option<Vec<String>>) -> Result<()>> = {
        let mut f = HashMap::new();
        f.insert("play"  , play);
        f.insert("pause" , pause);
        f.insert("stop"  , stop);
        f.insert("add"   , add);
    }
}

fn parse(cmd: lexer::Command) -> {
    if let Some(c) = cmd.command {

    }
}
