pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    if minefield.len() < 1 { // Base case
        return res;
    };

    for row in minefield {
        for column in row.chars() {}
    }

    return res;
    //  unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{:#?}\n", minefield);
}
