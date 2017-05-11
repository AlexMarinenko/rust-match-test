const VAR1: u16 = 1;
const VAR2: u16 = 2;
const VAR3: u16 = 3;
const VAR4: u16 = 4;

#[cfg(test)]
mod tests {

    // use {VAR1, VAR2, VAR3, VAR4};

    #[test]
    fn test_match() {
        let a = 4;
        match a {
            VAR1 => {
                println!("matched: 1");
            }
            VAR2 => {
                println!("matched: 2");
            }
            VAR3 => {
                println!("matched: 3");
            }
            VAR4 => {
                println!("matched: 4");
            }
            _ => {
                println!("not matched");
            }
        }
    }
}