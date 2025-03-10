use unicode_segmentation::UnicodeSegmentation::graphemes;


fn main(){
    let test = "uüu"
    let result = reverse(test);
    println!("{}", result);
}

fn reverse(input: &str) -> String{
    let mut result = String::new();
    for c in graphemes(input.as_str(), true).rev(){
        result.push_str(c);
    }
    return result.to_string();
}