fn main() {
    let profile = std::env::var("PROFILE").unwrap();
    let path=format!("./target/{}/AA.txt", profile);
    println!("{path}");
    std::fs::copy("./resources/AA.txt", path).unwrap();
}