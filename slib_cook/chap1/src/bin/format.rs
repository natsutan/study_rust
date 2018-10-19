
fn main() {
    let color = "red";
    let fav = format!("my fav = {}", color);
    println!("{}", fav);

    let fav_num = format!("fav num = {}", 42);
    println!("{}", fav_num);

    let duck = format!("{0} {0} {1} {0}", "duck", "goose");
    println!("{}", duck);

    let name = format!("{first}, {last}", first = "natu", last = "minoru");
    println!("{}", name)
}