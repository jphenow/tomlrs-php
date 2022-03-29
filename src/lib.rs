use std::fs;

use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, Zval};
use toml::Value as Toml;

fn convert(toml: Toml) -> Zval
{
    let mut zv = Zval::new();
    match toml {
        Toml::String(s) => {
            zv.set_string(&s, false).unwrap();
        },
        Toml::Integer(i) => zv.set_long(i),
        Toml::Float(f) => zv.set_double(f),
        Toml::Boolean(b) => zv.set_bool(b),
        Toml::Array(arr) => {
            let mut ht = ZendHashTable::with_capacity(
                arr.len().try_into().unwrap(),
            );

            for val in arr.into_iter() {
                ht.push(convert(val)).unwrap();
            }
            zv.set_hashtable(ht);
        }
        Toml::Table(table) => {
            // let newTable: HashMap<String, Zval> = HashMap::new();
            let mut ht = ZendHashTable::with_capacity(
                table.len().try_into().unwrap()
            );
            for (key, value) in table {
                ht.insert(&key, convert(value)).unwrap();
            }
            zv.set_hashtable(ht);
        }
        Toml::Datetime(dt) =>
        {
            zv.set_string(&dt.to_string(), false).unwrap();
        }
    }

    return zv;
}

/// Parses and returns a hash-map array of TOML to PHP
///
/// @param string $source TOML source file
///
/// @return array parsed TOML
#[php_function]
pub fn parse_toml(filename: String) -> Zval {
    //  Map<String, Toml> {
    // return toml::from_str(&source).unwrap();
    // let mut hm: HashMap<String, Toml> = HashMap::new();
    // let hashtable = ZendHashTable::new();

    // return toml_val.as_table().unwrap().clone();

    let file = fs::read_to_string(filename).unwrap();
    let toml_val: Toml = file.parse().unwrap();

    return convert(toml_val);
}

// Required to register the extension with PHP.
#[php_module]
pub fn phpmodule(module: ModuleBuilder) -> ModuleBuilder {
    module
}
