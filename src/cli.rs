use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version, about)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "Show rodo info.")]
    Info,

    #[clap(about = "Add a todo item.")]
    Add {
        #[clap(help = "The item content to add.")]
        content: Option<String>,
    },

    #[clap(about = "Remove a todo item.")]
    #[clap(visible_aliases = & ["rm"])]
    Remove {
        #[clap(help = "The item id to remove.")]
        id: Option<String>,
    },

    #[clap(about = "List all the todo items.")]
    #[clap(visible_aliases = & ["ls", "ll", "la"])]
    List,
}
