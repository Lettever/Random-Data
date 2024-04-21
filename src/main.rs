use ureq;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value, to_string_pretty};
use scraper::{Html, Selector};

/*
fn main() {
    let url = "https://pokemondb.net/location/unova-aspertia-city";
    let body = ureq::get(url).call().unwrap().into_string().unwrap();
    let s = stuff(body.split("\n").collect::<Vec<_>>());
	let mut keys: Vec<&str> = Vec::new();
	let mut json_obj = json!({/*"Route 1": {}*/});
	let qwe = HashMap::from([
		("Walking".to_string(), "Grass".to_string()),
		("Surfing".to_string(), "Water".to_string()),
		("Super Rod".to_string(), "Fishing".to_string()),
		("Interact".to_string(), "Others".to_string()),
		("Gift".to_string(), "Others".to_string()),
	]);
	let mut i = 0;
	while i < s.len() {
		let x = &s[i];
		if x.chars().nth(0).unwrap().is_ascii_uppercase() {
			if qwe.contains_key(x) {
				keys.clear();
				if let Some(a) = get_nested_object(&mut json_obj, &keys) {
					if a[&qwe[x]] == Value::Null || a[&qwe[x]][x] == Value::Null {
						a[&qwe[x]][x] = json!({});
					}
					keys.push(&qwe[x]);
					keys.push(x);
				}
			}
			else {
				let _ = keys.pop();
				if let Some(a) = get_nested_object(&mut json_obj, &keys) {
					if a[x] == Value::Null {
						a[x] = json!({});
					}
					keys.push(x);
				}
			}
		}
		else {
			if let Some(a) = get_nested_object(&mut json_obj, &keys) {
				if a[x] == Value::Null {
					a[x] = json!([]);
				}
				let b = a[x].as_array_mut().unwrap();
				b.push(json!([
					s[i + 1].parse::<bool>().unwrap(),
					s[i + 2].parse::<bool>().unwrap(),
					s[i + 3].parse::<bool>().unwrap(),
					s[i + 4].parse::<bool>().unwrap(),
					s[i + 5]
				]));
				i += 5;
			}
		}
		i += 1;
	}
	println!("{}", to_string_pretty(&json_obj).unwrap());
}*/
fn main() {
	let url = "https://bulbapedia.bulbagarden.net/wiki/Unova_Route_1";
    let document = ureq::get(url).call().unwrap().into_string().unwrap();
	let fragment = Html::parse_fragment(&document);
	
	let item_selector = Selector::parse(r#"table.roundy[cellspacing="2"]"#).unwrap();
	let pokemon_selector = Selector::parse(r#"table.roundy[width="500px"]"#).unwrap();
	let trainer_selector = Selector::parse(r#"table.roundy[align="left"]"#).unwrap();
	let route_selector = Selector::parse(r#"table.roundy >
	tbody > tr > td.roundy[style="border: 2px solid #4AA14D; background: #A2E0A3"]
	> table.roundy[width="100%"] > tbody > tr > td > table > tbody > tr > td > a"#).unwrap();
	
	//fix route selector
	//for now route_selector selects every route in the first table
	//remove all the non Unova ones
	
}

fn get_nested_object<'a>(json_value: &'a mut Value, keys: &[&str]) -> Option<&'a mut Value> {
    let mut current_value = json_value;

    for key in keys {
        if let Some(next_value) = current_value.get_mut(key) {
            current_value = next_value;
        } else {
            return None;
        }
    }

    Some(current_value)
}

//For the item table:
//	if it has 4 children, the 2nd one has the item and quantity and the 4th is the version
//	item: name = td > a; quantity = td
//	version: td > b > span > a > span