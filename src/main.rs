use ureq;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value, to_string_pretty};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
enum Type {
	None,
    Normal, Fire, Water,
    Electric, Grass, Ice,
    Fighting, Poison, Ground,
    Flying, Psychic, Bug,
    Rock, Ghost, Dragon,
    Dark, Steel, Fairy,
}
impl Type {
    const VALUES: [Self; 19] = [
        Self::None,
        Self::Normal, Self::Fire, Self::Water,
        Self::Electric, Self::Grass, Self::Ice,
        Self::Fighting, Self::Poison, Self::Ground,
        Self::Flying, Self::Psychic, Self::Bug,
        Self::Rock, Self::Ghost, Self::Dragon,
        Self::Dark, Self::Steel, Self::Fairy,
    ];
    fn from_string(s: &str) -> Option<Type> {
        match s.to_lowercase().as_ref() {
            "normal" => Some(Type::Normal),
            "fire" => Some(Type::Fire),
            "water" => Some(Type::Water),
            "electric" => Some(Type::Electric),
            "grass" => Some(Type::Grass),
            "ice" => Some(Type::Ice),
            "fighting" => Some(Type::Fighting),
            "poison" => Some(Type::Poison),
            "ground" => Some(Type::Ground),
            "flying" => Some(Type::Flying),
            "psychic" => Some(Type::Psychic),
            "bug" => Some(Type::Bug),
            "rock" => Some(Type::Rock),
            "ghost" => Some(Type::Ghost),
            "dragon" => Some(Type::Dragon),
            "dark" => Some(Type::Dark),
            "steel" => Some(Type::Steel),
            "fairy" => Some(Type::Fairy),
            _ => None,
        }
    }
	fn to_string(self) -> String {
		match self {
			Type::None => "None",
			Type::Normal => "Normal" ,
            Type::Fire => "Fire" ,
            Type::Water => "Water" ,
            Type::Electric => "Electric" ,
            Type::Grass => "Grass" ,
            Type::Ice => "Ice" ,
            Type::Fighting => "Fighting" ,
            Type::Poison => "Poison" ,
            Type::Ground => "Ground" ,
            Type::Flying => "Flying" ,
            Type::Psychic => "Psychic" ,
            Type::Bug => "Bug" ,
            Type::Rock => "Rock" ,
            Type::Ghost => "Ghost" ,
            Type::Dragon => "Dragon" ,
            Type::Dark => "Dark" ,
            Type::Steel => "Steel" ,
            Type::Fairy => "Fairy" ,
        }.to_string()
	}
}
#[derive(Debug)]
struct Pokemon {
    id: i32,
    name: String, type1: Type, type2: Type,
    hp: i32, atk: i32, def: i32,
    spatk: i32, spdef: i32, speed: i32,
}
impl Pokemon {
    fn new(id: i32, name: &str, type1: Type, type2: Type, hp: i32, atk: i32, def: i32, spatk: i32, spdef: i32, speed: i32,) -> Pokemon {
        Pokemon {
            id: id,
            name: name.to_string(),
            type1: type1,
            type2: type2,
            hp: hp,
            atk: atk,
            def: def,
            spatk: spatk,
            spdef: spdef,
            speed: speed,
        }
    }
    fn from_string(string: &str) -> Pokemon {
        let vals: Vec<String> = string.split(",").map(|x| x.to_string()).collect();
        Pokemon::new(
            vals[0].parse::<i32>().unwrap(), &vals[1],
            Type::from_string(&vals[2]).or(Some(Type::None)).unwrap(), Type::from_string(&vals[3]).or(Some(Type::None)).unwrap(),
            vals[4].parse::<i32>().unwrap(), vals[5].parse::<i32>().unwrap(),
            vals[6].parse::<i32>().unwrap(), vals[7].parse::<i32>().unwrap(),
            vals[8].parse::<i32>().unwrap(), vals[9].parse::<i32>().unwrap(),
        ) 
    }
}
fn main() {
    let url = "https://pokemondb.net/location/unova-route-1";
    let body = ureq::get(url).call().unwrap().into_string().unwrap();
    let s = stuff(body.split("\n").collect::<Vec<_>>());
	let mut keys = Vec::new();
	let mut json_obj = json!({/*"Route 1": {}*/});
	for x in s.iter() {
		if x == "Walking" {
			while keys.len() != 0 {
				keys.pop();
			}
			if let Some(a) = get_nested_object(&mut json_obj, &keys) {
				a["Grass"][x] = json!({});
				keys.push("Grass");
				keys.push(x);
			}
		}
		else if x == "Surfing" {
			while keys.len() != 0 {
				keys.pop();
			}
			if let Some(a) = get_nested_object(&mut json_obj, &keys) {
				a["Water"][x] = json!(x);
				keys.push("Water");
				keys.push(x);
			}
		}
		else if x == "Super Rod" {
			while keys.len() != 0 {
				keys.pop();
			}
			if let Some(a) = get_nested_object(&mut json_obj, &keys) {
				a["Fishing"][x] = json!({});
				keys.push("Fishing");
				keys.push(x);
			}
		}
	}
	println!("{:#?}", json_obj);
	println!("{:#?}", keys);
	let mut table = create_table();
	for (k, mut v) in &mut table {
		v.remove(&Type::None);
	}
	let mut file = File::create(r"..\data\matchup.json").unwrap();
	file.write(json!(table).to_string().as_bytes());
	//file.write(to_string_pretty(&json!(table)).unwrap().as_bytes());
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
/*
[
    "Walking",
    "patrat",
    "true",
    "true",
    "false",
    "false",
    "2-4",
    "lillipup",
    "true",
    "true",
    "false",
    "false",
    "2-4",
    "watchog",
    "false",
    "false",
    "true",
    "true",
    "56-59",
    "herdier",
    "false",
    "false",
    "true",
    "true",
    "56-59",
    "jigglypuff",
    "false",
    "false",
    "true",
    "true",
    "57",
    "Double Grass",
    "watchog",
    "true",
    "true",
    "false",
    "false",
    "32-35",
    "herdier",
    "true",
    "true",
    "false",
    "false",
    "32-34",
    "scraggy",
    "true",
    "true",
    "false",
    "false",
    "33-35",
    "watchog",
    "false",
    "false",
    "true",
    "true",
    "64-67",
    "herdier",
    "false",
    "false",
    "true",
    "true",
    "64-67",
    "jigglypuff",
    "false",
    "false",
    "true",
    "true",
    "65",
    "scrafty",
    "false",
    "false",
    "true",
    "true",
    "66-67",
    "Shaking/Bubbling spots",
    "audino",
    "true",
    "true",
    "false",
    "false",
    "2-4",
    "audino",
    "false",
    "false",
    "true",
    "true",
    "56-59",
    "wigglytuff",
    "false",
    "false",
    "true",
    "true",
    "59",
    "stoutland",
    "false",
    "false",
    "true",
    "true",
    "59",
    "dunsparce",
    "false",
    "false",
    "true",
    "true",
    "57",
    "Swarm",
    "farfetchd",
    "true",
    "true",
    "false",
    "false",
    "2",
    "Surfing",
    "basculin-red-striped",
    "true",
    "true",
    "false",
    "false",
    "5-15",
    "basculin-red-striped",
    "false",
    "false",
    "true",
    "true",
    "45-60",
    "Shaking/Bubbling spots",
    "basculin-red-striped",
    "true",
    "true",
    "false",
    "false",
    "5-20",
    "basculin-red-striped",
    "false",
    "false",
    "true",
    "true",
    "45-60",
    "Super Rod",
    "basculin-red-striped",
    "true",
    "true",
    "false",
    "false",
    "35-55",
    "basculin-red-striped",
    "false",
    "false",
    "true",
    "true",
    "40-70",
    "feebas",
    "true",
    "true",
    "false",
    "false",
    "35-55",
    "feebas",
    "false",
    "false",
    "true",
    "true",
    "50-70",
    "Shaking/Bubbling spots",
    "feebas",
    "true",
    "true",
    "false",
    "false",
    "35-60",
    "basculin-red-striped",
    "true",
    "true",
    "false",
    "false",
    "35-70",
    "feebas",
    "false",
    "false",
    "true",
    "true",
    "40-70",
    "basculin-red-striped",
    "false",
    "false",
    "true",
    "true",
    "40-60",
    "milotic",
    "true",
    "true",
    "false",
    "false",
    "45-70",
    "milotic",
    "false",
    "false",
    "true",
    "true",
    "50-70",
]
*/
fn stuff(lines: Vec<&str>) -> Vec<String>{
    let mut res = Vec::new();
    for l in lines.iter() {
        if l.starts_with("<h3>") {
            res.push(l[4 .. l.len() - 5].to_string());
        }
        else if l.starts_with("<source srcset=") {
            res.push(l[75 .. l.match_indices(".").collect::<Vec<_>>()[2].0].to_string());
        }
        else if l.starts_with(r#"<th colspan="9" class="cell-loc-status">"#) {
            res.push(l[40 .. l.len() - 5].to_string());
        }
        else if l.starts_with(r#"<td class="cell-loc-game"#) {
            let mut r = l.split("</td>").collect::<Vec<_>>();
            r.pop();
            for i in r {
                res.push((!i.contains("blank")).to_string());
            }
        }
        else if l.starts_with(r#"<td class="cell-num">"#) {
            res.push(l[21 .. l.len() - 5].to_string());
        }
    }
    return res;
}

fn create_table() -> HashMap<Type, HashMap<Type, f32>> {
    HashMap::from([
        (Type::Normal, create_matchup(
            &[],
            &[Type::Rock, Type::Steel],
            &[Type::Ghost],
        )),
        (Type::Fire, create_matchup(
            &[Type::Grass, Type::Ice, Type::Bug, Type::Steel],
            &[Type::Fire, Type::Water, Type::Rock, Type::Dragon],
            &[],
        )),
        (Type::Water, create_matchup(
            &[Type::Fire, Type::Ground, Type::Rock],
            &[Type::Water, Type::Grass, Type::Dragon],
            &[],
        )),
        (Type::Electric, create_matchup(
            &[Type::Water, Type::Flying],
            &[Type::Electric, Type::Grass, Type::Dragon],
            &[Type::Ground],
        )),
        (Type::Grass, create_matchup(
            &[Type::Water, Type::Ground, Type::Rock],
            &[Type::Fire, Type::Grass, Type::Ground, Type::Rock, Type::Dragon],
            &[],
        )),
        (Type::Ice, create_matchup(
            &[Type::Grass, Type::Ground, Type::Flying, Type::Dragon],
            &[Type::Ice],
            &[],
        )),
        (Type::Fighting, create_matchup(
            &[Type::Normal, Type::Rock, Type::Ice, Type::Dark, Type::Steel],
            &[Type::Poison, Type::Flying, Type::Psychic, Type::Bug, Type::Fairy],
            &[Type::Ghost],
        )),
        (Type::Poison, create_matchup(
            &[Type::Grass, Type::Fairy],
            &[Type::Poison, Type::Ground, Type::Rock, Type::Ghost],
            &[Type::Steel],
        )),
        (Type::Ground, create_matchup(
            &[Type::Fire, Type::Electric, Type::Poison, Type::Rock, Type::Steel],
            &[Type::Grass, Type::Bug],
            &[Type::Flying],
        )),
        (Type::Flying, create_matchup(
            &[Type::Grass, Type::Fighting, Type::Bug],
            &[Type::Electric, Type::Rock, Type::Steel],
            &[],
        )),
        (Type::Psychic, create_matchup(
            &[Type::Fighting, Type::Poison],
            &[Type::Psychic, Type::Steel],
            &[Type::Dark],
        )),
        (Type::Bug, create_matchup(
            &[Type::Grass, Type::Psychic, Type::Dark],
            &[Type::Fire, Type::Fighting, Type::Poison, Type::Flying, Type::Ghost, Type::Steel, Type::Fairy],
            &[],
        )),
        (Type::Rock, create_matchup(
            &[Type::Fire, Type::Ice, Type::Flying, Type::Bug],
            &[Type::Fighting, Type::Ground, Type::Steel],
            &[],
        )),
        (Type::Ghost, create_matchup(
            &[Type::Psychic, Type::Ghost],
            &[Type::Dark],
            &[Type::Normal],
        )),
        (Type::Dragon, create_matchup(
            &[Type::Dragon],
            &[Type::Steel],
            &[Type::Fairy],
        )),
        (Type::Dark, create_matchup(
            &[Type::Psychic, Type::Ghost],
            &[Type::Fighting, Type::Dark, Type::Fairy],
            &[],
        )),
        (Type::Steel, create_matchup(
            &[Type::Ice, Type::Rock, Type::Fairy],
            &[Type::Fire, Type::Water, Type::Electric, Type::Steel],
            &[],
        )),
        (Type::Fairy, create_matchup(
            &[Type::Fighting, Type::Dragon, Type::Dark],
            &[Type::Fire, Type::Poison, Type::Steel],
            &[],
        )),    
    ])
}
fn create_matchup(super_effective: &[Type], not_effective: &[Type], immune: &[Type]) -> HashMap::<Type, f32> {
    let mut res = HashMap::new();
    for t in Type::VALUES {
        res.insert(t, 1.0);
    }
    for t in super_effective.iter() {
        res.insert(*t, 2.0);
    }
    for t in not_effective.iter() {
        res.insert(*t, 0.5);
    }
    for t in immune.iter() {
        res.insert(*t, 0.0);
    }
    return res;
}
fn get_matchup(matchup_table: &HashMap<Type, HashMap<Type, f32>>, t1: Type, t2: Type, t3: Type) -> f32 {
    matchup_table[&t1][&t2] * matchup_table[&t1][&t3]
}
fn lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    return reader.lines().map(|line| line.unwrap()).collect();
}
/*
*/