#[derive(Debug)]
pub struct Command {
    command: Option<String>,
    args: Option<String>
}

pub fn lex(input: &str) -> Command {
    let parts = input.splitn(2, ' ').collect::<Vec<&str>>();
    let mut parts_iter = parts.iter();

    let command = parts_iter.next().map(|x| x.to_string());
    let args = parts_iter.last();

    if let Some(a) = args {
        return Command {
            command: command,
            args: Some(a.to_string())
        }
    } else {
        return Command {
            command,
            args: None
        }
    }
}
