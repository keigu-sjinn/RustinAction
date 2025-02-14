struct Hostname(String);

fn main() {
    let ordinary_string = String::from("localhost");
    let host = Hostname(ordinary_string.clone());

    connect(host);
}

fn connect(host: Hostname) {
    println!("connected to {}", host.0);
}
