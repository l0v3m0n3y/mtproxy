use rand::prelude::*;
use std::fs::File;
use std::io::Write;
use std::error::Error;


const SECRETS: &[&str] = &[
    "5659e666ad01e07fed3995306dbe785c",
    "6cbe952cf6ce201aad31a365f88f6834",
    "02567895943d8069be957f7058fa8aed",
    "fd3eeaff98d833deecc026f6d641ac97",
    "70117414ed47078c62f1dd5046c938db",
    "39682ffd39beefcc991d12c832866bca",
    "cef63673e0225e4352204f25e87fbb7d",
    "3f1bf2a2699b3f2d90da461534c3c54a",
    "fdc925749cb4089b239782fb95106f88",
    "7b5e8f646522d883fd50d2a1b1855f2d",
    "f6fe059664c24962c99fbcc32553249e",
    "bf2e13d1f062eaa260df32dd5768b3e7",
    "c1626f14f79474d7d27ae1461d2d1c70",
    "28ed0c814a97362df6fed7339922e795",
    "b968de5d8c88024af77ee0ed494abb74",
    "a8e84b0bdd918ad1aa18e8380b24690a",
];

const DOMAIN: &str = "proxytg.ink";
const PORT: u16 = 443;
const PREFIXES: &[&str] = &["proxy-telegram-", "proxy-", "proxytg-"];

fn token_hex() -> String {
    let mut bytes = [0u8; 8];
    let mut rng = thread_rng();
    rng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

fn pick<T>(arr: &[T]) -> &T {
    let mut rng = thread_rng();
    &arr[rng.gen_range(0..arr.len())]
}

fn hex_encode(s: &str) -> String {
    hex::encode(s.as_bytes())
}

fn generate_proxy_link() -> String {
    let raw = pick(SECRETS);
    let domain_hex = hex_encode(DOMAIN);
    let secret = format!("ee{}{}", raw, domain_hex);
    
    let sub_prefix = pick(PREFIXES);
    let subdomain = format!("{}{}", sub_prefix, token_hex());
    
    format!(
        "tg://proxy?server={}.{}&port={}&secret={}",
        subdomain, DOMAIN, PORT, secret
    )
}

fn write_links_to_file(filename: &str, links: &[String]) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    for link in links {
        writeln!(file, "{}", link)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut links = Vec::new();
    for _ in 1..=10 {
        links.push(generate_proxy_link());
    }
    
    write_links_to_file("proxy_links.txt", &links)?;

    let personal_link = generate_proxy_link();
    write_links_to_file("personal_proxy.txt", &[personal_link])?;
    
    Ok(())
}
