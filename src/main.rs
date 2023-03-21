fn main() {
    let x:String = String::from("Hello world");
    let y:String = String::from("Hello world of rust");
    println!("Longest str is: {}",longest(x.as_str(), y.as_str()))
}


fn longest<'a>(x:&'a str, y:&'a str) -> &'a str
{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
