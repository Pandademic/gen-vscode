trait StrExt {
    fn from_end(&self, n: usize) -> char;
    fn remove_last(&self) -> &str;
}

impl<'a> StrExt for &'a str {
    fn from_end(&self, n: usize) -> char {
        self.chars().rev().nth(n).expect("Index out of range in 'from_end'")
    }
    fn remove_last(&self) -> &str {
        match self.char_indices().next_back() {
            Some((i, _)) => &self[..i],
            None => self,
        }
    }
}
fn remove_first_and_last(val: &str) -> &str {
    let mut chars = val.chars();
    chars.next();
    chars.next_back();
    return chars.as_str();
}
fn remove_first(val: &str) -> &str {
    let mut chars = val.chars();
    chars.next();
    return chars.as_str();
}
pub fn eval(what: &str) -> (Vec<String>,Vec<String>) {
    let lines: Vec<&str> = what.split("\n").collect();
    let mut settings:Vec<String> = Vec::new();
    let mut exts: Vec<String> = Vec::new();
    for line in lines {
        if line == "" || line == " " || line == "  " || line == "\r"{
            continue // it's an empty line of some sort
        }
        let semi_colon:char = ';';
        let begin_paren:char = '(';
        let end_paren:char = ')';
        if line.chars().nth(0).unwrap() == semi_colon {
            continue // It's a comment
        }
        if line.chars().nth(0).unwrap() != begin_paren {
            println!("line:{}",line);
            panic!("no starting parenthesis found!")
        }
        if line.from_end(1) != end_paren {
            panic!("no ending parenthesis found!")
        }
        let mut line_v2 = remove_first_and_last(line); // seems to do nothing
        
        let chars_of_line_v2 = line_v2.chars();
        // use-ext : 7 chars , set-key : 7 chars
        let first_few: String = chars_of_line_v2.into_iter().take(7).collect();
        if first_few != "use-ext" && first_few != "set-key" {
            panic!("unrecognized call!")
        }

        for _ in 0..7 {
            line_v2 = remove_first(line_v2);
        }
        let line_v2 = line_v2.remove_last();
        let line_v2 = remove_first(line_v2);
        if first_few == "use-ext" {
           exts.push(line_v2.to_string());
        }
        else if first_few == "set-key" {
            settings.push(line_v2.to_string()) // or whats left of it( the setting key and value)
        }

            
    }
    return (settings,exts)
}
