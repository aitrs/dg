use clap::Parser;

/// Utility to generate mock data using handlebars templates
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Number of items to generate
    #[arg(short, long, default_value_t = 10)]
    pub count: usize,

    /// Template files paths.
    pub templates: Vec<String>,

    /// Iterable items prefix in template
    #[arg(short, long, default_value_t=String::from("item"))]
    pub prefix: String,

    /// Output to files
    /// every template is mapped sequencially to each output file. 
    /// If a template cannot be mapped to a specific file, it is rendered on stdout
    #[arg(short, long)]
    pub output: Vec<String>,

    /// Random string size
    #[arg(short, long, default_value_t = 10)]
    pub random_string_size: usize,

    /// Definition flag
    #[arg(short, long)]
    pub definitions: Vec<String>,

    /// Definitions file (overrides definition flag)
    #[arg(long)]
    pub definitions_file: Option<String>,
}
