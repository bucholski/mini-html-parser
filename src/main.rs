pub mod tokenize;
pub mod structs;
#[allow(unused)]
fn main() {
    let example = String::from(
        r#"<!doctype html><html lang="en"><head><meta charset="utf-8"/><link href="https://fonts.googleapis.com/css2?family=Quicksand:wght@400;600&display=swap" rel="stylesheet"/><meta name="viewport" content="width=device-width,initial-scale=1"/><meta name="description" content="Michał Bucholski | Frontend developer"/><link rel="manifest" href="/portfolio/manifest.json"/><title>Misha Bucholski - Frontend Developer</title><script defer="defer" src="/portfolio/static/js/main.07675d55.js"></script><link href="/portfolio/static/css/main.33eb7f92.css" rel="stylesheet"></head><body><noscript>You need to enable JavaScript to run this app.</noscript><div id="root"></div></body></html>"#
    );
}
