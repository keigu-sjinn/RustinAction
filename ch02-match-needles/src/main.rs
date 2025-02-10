fn main() {
    let needle = 0o204;
    let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];

    // for item in &haystack {
    //     let result = match item {
    //         42 | 132 => "hit!",
    //         _ => "miss!",
    //     };

    //     if result == "hit!" {
    //         println!("{}: {}", item, result);
    //     }
    // }

    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }
}
