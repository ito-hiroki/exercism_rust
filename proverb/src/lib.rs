pub fn build_proverb(list: &[&str]) -> String {
    let mut ans = String::new();
    for i in 0..list.len() {
        if i != list.len() - 1 {
            ans.push_str(&format!(
                "For want of a {} the {} was lost.\n",
                list[i],
                list[i + 1]
            ));
        } else {
            ans.push_str(&format!("And all for the want of a {}.", list[0]));
        }
    }
    ans
}
