use actix_web::http::header;
use actix_web::{web, HttpResponse, Responder};
use futures::future::join_all;
use rand::Rng;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;

const MAX_POKEMON_COUNT: i32 = 1025;

#[derive(Deserialize)]
pub struct SearchParams {
    pub id_or_name: String,
}

#[derive(Deserialize)]
pub struct RandomParams {
    pub n: i32,
}

fn validate_pokemon_identifier(input: &str) -> Result<(), &'static str> {
    if input.is_empty() {
        return Err("Input cannot be empty");
    }

    if input.len() > 100 {
        return Err("Input too long");
    }

    let re = Regex::new(r"^[a-zA-Z0-9\-_]+$").unwrap();
    if !re.is_match(input) {
        return Err("Invalid characters in input");
    }

    Ok(())
}

async fn fetch_pokemon(client: &reqwest::Client, id_or_name: &str) -> Result<Value, HttpResponse> {
    if let Err(msg) = validate_pokemon_identifier(id_or_name) {
        return Err(HttpResponse::BadRequest().body(msg));
    }

    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", id_or_name);

    let pokemon_data: Value = client
        .get(&url)
        .send()
        .await
        .map_err(|_| HttpResponse::NotFound().body("Pokemon not found"))?
        .json()
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Failed to parse data"))?;

    let species_url = pokemon_data["species"]["url"]
        .as_str()
        .ok_or_else(|| HttpResponse::InternalServerError().body("No species URL found"))?;

    let species_data: Value = client
        .get(species_url)
        .send()
        .await
        .map_err(|_| HttpResponse::NotFound().body("Species not found"))?
        .json()
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Failed to parse species"))?;

    Ok(serde_json::json!({
        "name": pokemon_data["name"],
        "image_url": pokemon_data["sprites"]["front_default"],
        "color": species_data["color"]["name"],
        "types": pokemon_data["types"],
        "species": pokemon_data["species"],
        "id": pokemon_data["id"],
        "weight": pokemon_data["weight"],
        "height": pokemon_data["height"],
        "abilities": pokemon_data["abilities"],
    }))
}

pub async fn search(query: web::Query<SearchParams>) -> impl Responder {
    let client = reqwest::Client::new();
    match fetch_pokemon(&client, &query.id_or_name).await {
        Ok(pokemon) => HttpResponse::Ok()
            .insert_header((header::CACHE_CONTROL, "public, max-age=10"))
            .json(pokemon),
        Err(e) => {
            println!("Error: {:?}", e);
            e
        }
    }
}

pub async fn get_n_random(query: web::Query<RandomParams>) -> impl Responder {
    if query.n > 15 {
        return HttpResponse::BadRequest().body("n must be less than 15");
    }

    let client = reqwest::Client::new();
    let mut rng = rand::thread_rng();

    let ids: Vec<String> = (0..query.n)
        .map(|_| rng.gen_range(1..MAX_POKEMON_COUNT).to_string())
        .collect();

    let promises = ids.iter().map(|id| fetch_pokemon(&client, id));
    let results = join_all(promises).await;

    let pokemons: Vec<_> = results.into_iter().filter_map(Result::ok).collect();
    HttpResponse::Ok().json(pokemons)
}
