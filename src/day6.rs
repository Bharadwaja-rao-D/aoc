pub fn day6(lines: String) {
    let mut store: Vec<usize> = Vec::with_capacity(14);
    let chars: Vec<char> = lines.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        if store.len() == 14 {
            break;
        } else {
            if let Some(present) = store.iter().position(|ele| chars[*ele] == chars[i]) {
                i = store[present];
                store.clear();
            } else {
                store.push(i);
            }
        }
        i += 1;
    }

    println!("{}", store.last().unwrap() + 1);
}
