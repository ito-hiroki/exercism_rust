pub fn verse(n: u32) -> String {
    let bottle = num_bottles(n);
    let mut ans: String = format!(
        "{} of beer on the wall, {} of beer.\n",
        bottle,
        bottle.to_lowercase()
    );
    match n {
        0 => ans.push_str("Go to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => {
            ans.push_str("Take it down and pass it around, no more bottles of beer on the wall.\n")
        }
        _ => ans.push_str(&format!(
            "Take one down and pass it around, {} of beer on the wall.\n",
            num_bottles(n - 1)
        )),
    }
    ans
}

fn num_bottles(n: u32) -> String {
    match n {
        0 => String::from("No more bottles"),
        1 => String::from("1 bottle"),
        x => String::from(format!("{} bottles", x)),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    // let mut song = String::new();
    // for i in (end + 1..start + 1).rev() {
    //     song.push_str(&(verse(i) + "\n"));
    // }
    // song.push_str(&verse(end));
    // song
    (end..start + 1)
        .rev()
        .map(|x| verse(x))
        .collect::<Vec<_>>()
        .join("\n")
}
