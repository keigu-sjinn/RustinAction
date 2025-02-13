use regex::Regex;

fn main() {
    let re = Regex::new(r"picture").unwrap();
    let quote =
        "\
Every face, every shap, bedroom windows, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of pages?";

    for line in quote.lines() {
        let contains_re = re.find(line);
        match contains_re {
            Some(n) => println!("{}:{:?}", line, n),

            None => (),
        }
    }
}
