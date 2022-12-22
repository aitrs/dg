use clap::Parser;
use datagen::{cli::Cli, defparser::definition_parser, item::items_from_cli};
use handlebars::Handlebars;
use std::{
    collections::BTreeMap,
    error::Error,
    fs::{self, File},
};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut items_map = BTreeMap::new();
    let mut handlebars = Handlebars::new();

    let definitions = if let Some(definition_file) = &cli.definitions_file {
        let contents = fs::read_to_string(definition_file)?;
        definition_parser::definitions(&contents)?
    } else if !cli.definitions.is_empty() {
        cli.definitions
            .iter()
            .map(|def| definition_parser::definition(def.as_str()).unwrap())
            .collect()
    } else {
        vec![]
    };

    items_map.insert(&cli.prefix, items_from_cli(&cli, &definitions));
    let out_vector = &cli.output.clone();
    let mut out_iter = out_vector.iter();

    for template in &cli.templates {
        handlebars.register_template_file("template", template)?;

        if let Some(output_file_name) = out_iter.next() {
            let mut output_file = File::create(output_file_name)?;
            handlebars.render_to_write("template", &items_map, &mut output_file)?;
        } else {
            println!("{}", handlebars.render("template", &items_map)?);
        }
    }

    Ok(())
}
