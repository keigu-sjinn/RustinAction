fn main() {
    let search_term = "picture";
    let quote = "\
Every face, every shap, bedroom windows, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of pages?";

    for (index, line) in quote.lines().enumerate() {
        let line_num = index + 1;
        if line.contains(search_term) {
            println!("{}: {}", line_num, line);
        }
    }
}
