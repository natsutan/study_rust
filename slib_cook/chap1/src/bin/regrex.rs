extern crate regex;


fn main()
{
    use regex::Regex;

    let date_regx = Regex::new(r"^\d{2}.\d{2}.\d{4}$").expect("Fail");
    let data = "15.10.2017";

    let is_date = date_regx.is_match(data);
    println!("Is {} data? {}", data, is_date);

    let date_regex = Regex::new(r"(\d{2}).(\d{2}).(\d{4})").expect("Fail");
    let text_with_date = "Alan on 23.06.1912 and 14.11.2017";

    for cap in date_regex.captures_iter(text_with_date) {
        println!("found {} {} {} {} ", &cap[0], &cap[1], &cap[2], &cap[3]);
    }

    println!("original {}", text_with_date);
    let indian = date_regex.replace_all(text_with_date, "$1-$2-$3");
    println!("indian {}", indian);

}