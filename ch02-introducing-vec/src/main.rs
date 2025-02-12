fn main() {
    let ctx_lines = 2;
    let needle = "oo";
    let quote = vec![
        "Every face, every shop,", 
        "bedroom windows, public-house, and",
        "dark square is a picture", 
        "feverishly turned--in search of what?",
        "It is the same with books.",
        "What do we seek", 
        "through millions of pages?"
    ];

    // 第一次遍历：收集所有匹配行的索引
    let tags: Vec<usize> = quote.iter()
        .enumerate()
        .filter(|(_, &line)| line.contains(needle))
        .map(|(i, _)| i)
        .collect();

    if tags.is_empty() { 
        return;
    }

    // 使用HashSet记录所有需要显示的行号
    let mut lines_to_show = std::collections::HashSet::new();
    for &tag in &tags {
        // 将匹配行及其上下文加入集合
        (tag.saturating_sub(ctx_lines)..=tag + ctx_lines)
            .filter(|&i| i < quote.len())
            .for_each(|i| { lines_to_show.insert(i); });
    }

    // 按顺序输出结果
    let mut output = Vec::new();
    for (i, line) in quote.iter().enumerate() {
        if lines_to_show.contains(&i) {
            output.push(format!("{}: {}", i + 1, line.trim()));
        }
    }
    
    println!("{}", output.join("\n"));
}
