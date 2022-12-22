use std::collections::BTreeMap;

use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Serialize;

use crate::{
    cli::Cli, defparser::Definitions, firstnames::generate_firstnames,
    lastnames::generate_lastnames,
};

#[derive(Serialize)]
pub struct Item {
    random: i32,
    randstr: String,
    increment: usize,
}

impl Item {
    pub fn new(increment: usize, string_size: usize) -> Self {
        Item {
            random: thread_rng().gen(),
            randstr: thread_rng()
                .sample_iter(&Alphanumeric)
                .take(string_size)
                .map(char::from)
                .collect(),
            increment,
        }
    }
}

pub fn items_from_cli(cli: &Cli, definitions: &Definitions) -> Vec<BTreeMap<String, String>> {
    let mut items = Vec::new();
    let firstnames = generate_firstnames();
    let lastnames = generate_lastnames();
    let mut rng = thread_rng();

    for index in 0..cli.count {
        let mut curmap = BTreeMap::new();
        curmap.insert(
            "random".to_string(),
            format!("{}", rng.clone().gen::<i64>()),
        );
        curmap.insert(
            "randstr".to_string(),
            rng.clone()
                .sample_iter(&Alphanumeric)
                .take(cli.random_string_size)
                .map(char::from)
                .collect(),
        );
        curmap.insert("increment".to_string(), format!("{}", index));
        curmap.insert(
            "firstname".to_string(),
            firstnames[rng.gen_range(0..firstnames.len())].to_string(),
        );
        curmap.insert(
            "lastname".to_string(),
            lastnames[rng.gen_range(0..lastnames.len())].to_string(),
        );

        for d in definitions {
            let (key, value) = d.run(index);
            curmap.insert(key, format!("{}", value));
        }

        items.push(curmap);
    }

    items
}
