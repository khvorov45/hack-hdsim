pub fn tokenise_hdl(contents: String) {
    println!("String to tokenise:\n{}", contents);
    let temp = contents.split(' ');
    println!("Tokens:\n");
    for tok in temp {
        println!("{}", tok);
    }
}
