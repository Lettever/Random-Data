use ureq;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value, to_string_pretty};
use scraper::{Html, Selector, ElementRef};

struct Item {
	name: String,
	quantity: u8,
	games: [bool; 4]
}

fn main() {
	let url = "https://bulbapedia.bulbagarden.net/wiki/Unova_Route_1";
    let document = ureq::get(url).call().unwrap().into_string().unwrap();
	let fragment = Html::parse_fragment(&document);
	
	let item_selector = Selector::parse(r#"table.roundy[cellspacing="2"]"#).unwrap();
	let pokemon_selector = Selector::parse(r#"table.roundy[width="500px"]"#).unwrap();
	let trainer_selector = Selector::parse(r#"table.roundy[align="left"]"#).unwrap();
	let route_selector = Selector::parse(r#"table.roundy > tbody > tr >
	td.roundy[style="border: 2px solid #4AA14D; background: #A2E0A3"] >
	table.roundy[width="100%"] > tbody > tr > td > table > tbody > tr > td > a"#).unwrap();
	
	println!("{}", fragment.select(&route_selector).collect::<Vec<_>>().len() == 7);
	let routes: Vec<_> = {
		let mut res = Vec::new();
		for i in fragment.select(&route_selector) {
			println!("{:?}", i.value());
			if i.value().attr("href").unwrap().starts_with("/wiki/Unova") {
				res.push(i);
			}
		}
		res
	};
	for i in routes.iter() {
		println!("{:?}", i.value());
	}
	get_items(url);
	//fix route selector
	//for now route_selector selects every route in the first table
	//remove all the non Unova ones
	//for route one it returns 7 but it should be 5 (4 uniques and 1 repeated)
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
fn get_items(url: &str) /*-> Vec<ElementRef<'_>> */{
	let document = ureq::get(url).call().unwrap().into_string().unwrap();
	let fragment = Html::parse_fragment(&document);
	let selector = Selector::parse(r#"table.roundy[cellspacing="2"] > tbody > tr"#).unwrap();
	//let s2 = Selector::parse(r#"td"#).unwrap();
	let table = {
		let table = fragment.select(&selector).collect::<Vec<_>>();
		table[1..table.len() - 1].to_vec()
	};
	for i in table {
		println!("{:?}", i.value());
	}
}
//For the item table:
//	if it has 4 children, the 2nd one has the item and quantity and the 4th is the version
//	item: name = td > a; quantity = td
//	version: td > b > span > a > span

/*
<tr>
<tr>
<tr>
<tr>
<tr>
<tr>
<tr>
<tr>
<tr>
<tr>
<tr>
<tr>
<tr>
*/

/*
Route 1
Route 2
Route 3
Route 4
Route 5
Route 6
Route 7
Route 8
Route 9
Route 10
Route 11
Route 12
Route 13
Route 14
Route 15
Route 16
Route 17
Route 18
Route 19
Route 20
Route 21
Route 22
Route 23
Abundant Shrine
Accumula Town
Anville Town
Aspertia City
Battle Subway
Black City
Castelia City
Castelia Sewers
Cave of Being
Celestial Tower
Challenger's Cave
Chargestone Cave
Clay Tunnel
Cold Storage
Desert Resort
Dragonspiral Tower
Dreamyard
Driftveil City
Driftveil Drawbridge
Floccesy Ranch
Floccesy Town
Gear Station
Giant Chasm
High Link
Humilau City
Icirrus City
Lacunosa Town
Lentimas Town
Liberty Island
Lostlorn Forest
Marine Tube
Marvelous Bridge
Mistralton Cave
Mistralton City
Moor of Icirrus
N's Castle
Nacrene City
Nature Preserve
Nature Sanctuary
Nimbasa City
Nuvema Town
Opelucid City
P2 Laboratory
Pinwheel Forest
Plasma Frigate
Pledge Grove
Pokémon League
Pokémon World Tournament
Relic Castle
Relic Passage
Reversal Mountain
Roaming Unova
Seaside Cave
Skyarrow Bridge
Strange House
Striaton City
Tubeline Bridge
Twist Mountain
Undella Bay
Undella Town
Underground Ruins
Undersea Ruins
Unity Tower
Victory Road
Village Bridge
Virbank City
Virbank Complex
Wellspring Cave
White Forest
*/