use rustyline::config::{ Builder, Config, CompletionType, EditMode };
use rustyline::Editor;

fn generate_config() -> Config {
    let line_config: Config = Builder::new()
        .max_history_size(100)
        .history_ignore_dups(true)
        .history_ignore_space(true)
        .completion_type(CompletionType::Circular)
        .edit_mode(EditMode::Emacs)
        .build();
    return line_config;
}

pub fn create_readline() -> Editor<()> {
    let line_config = generate_config();
    let rl = Editor::<()>::with_config(line_config);
    return rl;
}
