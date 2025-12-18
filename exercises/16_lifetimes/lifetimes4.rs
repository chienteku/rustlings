fn split<'a, 'b>(text: &'a str, delimiter:&'b str) -> Vec<&'a str> {
    let mut last_split = 0;
    let mut matches: Vec<&str> = vec![];
    for i in 0..text.len(){
        if i < last_split {
            continue;
        }
        if text[i..].starts_with(delimiter){
            matches.push(&text[last_split..i]);
            last_split = i + delimiter.len();
        }
    }
    if last_split < text.len() {
        matches.push(&text[last_split..]);
    }
    matches

}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        assert_eq!(split("a,b,c", ","), vec!["a", "b", "c"]);
        assert_eq!(split("a, b, c", ", "), vec!["a", "b", "c"]);
    }
}
