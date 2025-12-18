
struct TextSplitter<'b>{
    delimiter: &'b str
}

impl<'b> TextSplitter<'b> {
    fn split<'a>(&self, text: &'a str) -> Vec<&'a str> {
        // Option 1: Manual implementation
        // let mut last_split = 0;
        // let mut matches: Vec<&str> = vec![];
        // for i in 0..text.len(){
        //     if i < last_split {
        //         continue;
        //     }
        //     if text[i..].starts_with(self.delimiter){
        //         matches.push(&text[last_split..i]);
        //         last_split = i + self.delimiter.len();
        //     }
        // }
        // if last_split < text.len() {
        //     matches.push(&text[last_split..]);
        // }
        // matches

        // Option 2: use standard library function
        text.split(self.delimiter).collect()
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let text1 = "a, b, c";
        let splitter = TextSplitter { delimiter: "," };
        assert_eq!(splitter.split(text1), vec!["a", " b", " c"]);
        
        let splitter2 = TextSplitter { delimiter: ", " };
        assert_eq!(splitter2.split(text1), vec!["a", "b", "c"]);
    
        let text2 = "一 二 三";
        let splitter3 = TextSplitter { delimiter: " " };
        assert_eq!(splitter3.split(text2), vec!["一", "二", "三"]);
    }
}
