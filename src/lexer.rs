#[derive(Debug)]
pub struct Command {
    command: Option<String>,
    args: Option<Vec<String>>
}

pub fn lex(input: &str) -> Command {
    let parts = input.split(' ').collect::<Vec<&str>>();
    let mut parts_iter = parts.iter();
    let command = parts_iter.next().map(|&x| String::from(x.clone()));
    let args: Vec<String> = parts_iter.map(|&x| x.to_string()).collect();
    if args.len() != 0 {
        return Command {
            command,
            args: Some(args)
        };
    } else {
        return Command {
            command,
            args: None
        }
    }
}
