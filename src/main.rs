use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty, Value};
use std::collections::btree_map::Values;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use ureq;

struct Item {
    name: String,
    quantity: u8,
    games: [bool; 4],
}

fn main() {
    let url = "https://bulbapedia.bulbagarden.net/wiki/Unova_Route_1";
    let document = ureq::get(url).call().unwrap().into_string().unwrap();
    let fragment = Html::parse_fragment(&document);
}

fn get_items(url: &str) -> Value {
    let document = ureq::get(url).call().unwrap().into_string().unwrap();
    let fragment = Html::parse_fragment(&document);
    let selector = Selector::parse(r#"table.roundy[cellspacing="2"] > tbody > tr"#).unwrap();

    let table = {
        let table = fragment.select(&selector).collect::<Vec<_>>();
        table[1..table.len() - 1].to_vec()
    };

    return json!({});
}
fn get_pokemon(url: &str) -> Value {
    let document = ureq::get(url).call().unwrap().into_string().unwrap();
    let fragment = Html::parse_fragment(&document);
    let selector = Selector::parse(r#"table.roundy[width="500px"]"#).unwrap();

    return json!({});
}
fn get_trainer(url: &str) -> Value {
    let document = ureq::get(url).call().unwrap().into_string().unwrap();
    let fragment = Html::parse_fragment(&document);
    let selector = Selector::parse(r#"table.roundy[align="left"]"#).unwrap();

    return json!({});
}
fn get_routes(url: &str) -> Value {
    let document = ureq::get(url).call().unwrap().into_string().unwrap();
    let fragment = Html::parse_fragment(&document);
    let selector = Selector::parse(
        r#"table.roundy > tbody > tr >
		td.roundy[style="border: 2px solid #4AA14D; background: #A2E0A3"] >
		table.roundy[width="100%"] > tbody > tr > td > table > tbody > tr > td > a"#,
    )
    .unwrap();

    let table = {
        let table = fragment.select(&selector).collect::<Vec<_>>();
        table[1..table.len() - 4].to_vec()
    };

    return json!({});
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
