use std::net::IpAddr;

fn main() {
    let home: IpAddr = "127.0.0.999"
        .parse()
        .expect("Hardcoded IP address should be valid");

    println!("home = {home}");
}
