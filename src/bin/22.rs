#[tokio::main]
async fn main() {
    let text = reqwest::get("https://projecteuler.net/resources/documents/0022_names.txt")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let mut names = text
        .split(',')
        .map(|name| name.trim_matches('"'))
        .collect::<Vec<_>>();
    names.sort();
    let result = names
        .into_iter()
        .enumerate()
        .map(|(i, name)| (i as u32 + 1) * sum_name(name))
        .sum::<u32>();
    println!("{}", result);
}

fn sum_name(name: &str) -> u32 {
    name.chars().map(|c| c as u32 - 'A' as u32 + 1).sum()
}
