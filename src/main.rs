use reqwest::blocking::Client;
use serde_json::Value;
use std::env;

fn main() {
    let token = env::var("TELEGRAM_TOKEN").expect("Token hilang");
    let chat_id = env::var("TELEGRAM_CHAT_ID").expect("Chat ID hilang");
    let client = Client::new();

    let url = "https://data.bmkg.go.id/DataMKG/TEWS/autogempa.json";
    let res = client.get(url).send().unwrap().json::<Value>().unwrap();
    
    let g = &res["Infogempa"]["gempa"];
    let pesan = format!(
        "⚠️ GEMPA TERBARU\n\n📍 Wilayah: {}\n📏 Magnitudo: {}\n⏱ Waktu: {}\n🌊 Potensi: {}",
        g["Wilayah"], g["Magnitude"], g["Jam"], g["Potensi"]
    );

    let tele_url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    client.post(tele_url)
        .form(&[("chat_id", &chat_id), ("text", &pesan)])
        .send()
        .unwrap();

    println!("Pesan terkirim!");
}
