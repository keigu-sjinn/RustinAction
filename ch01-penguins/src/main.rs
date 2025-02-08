fn main() {
    let penguins = "\
common name, length (cm)
Little penguins, 33
Yellow-eyed penguins, 65
Fiordland penguins, 60
Invalid, data
    ";

    let records = penguins.lines().map(|rec| rec.trim());

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {:.1}cm", name, length);
        }
    }
}
