fn main()
{
    let mut fruits = Vec::new();
    fruits.push("banana");
    fruits.push("tomato");
    fruits.push("pear");

    let last = fruits.pop();
    if let Some(last) = last {
        println!("removed {} from {:?}", last, fruits);
    }

    //insert
    fruits.insert(1, "grape");
    println!("{:?}", fruits);
    //swap
    fruits.swap(2,0);
    println!("{:?}", fruits);

    let first = fruits.first().unwrap();
    println!("first = {}", first);

    let last = fruits.last();
    if let Some(last) = last {
        println!("last = {}", last);
    }

    let second = fruits[1];
    println!("second = {}", second);

    let branch_of_zeros = vec![0; 5];
    println!("branch_of_zeros = {:?}", branch_of_zeros);

    //Filter
    let mut names = vec!["Alex", "Jane", "Aaron", "Daniel"];
    names.retain(|name| name.starts_with('A'));
    println!("names = {:?}", names);

    println!("{}", names.contains(&"Alex"));

    //重複の削除
    let mut nums = vec![1, 2, 2, 2, 3, 3, 4, 5, 1,  5];
    nums.sort();
    nums.dedup();
    println!("deduped {:?}", nums);

    nums.reverse();
    println!("reverse {:?}", nums);

    let mut alphabet = vec!['a', 'b', 'c'];
    for l in alphabet.drain(..2) {
        println!("letter {}", l);
    }
    println!("drained {:?}", alphabet);

    alphabet.clear();
    println!("empty? {}", alphabet.is_empty());

    let mut colors = vec!["red", "green", "yellow", "blue"];
    println!("colors {:?}", colors);
    let half = colors.len() / 2;
    let mut second_half = colors.split_off(half);
    println!("split_off colors = {:?}, second half = {:?}", colors, second_half);

    colors.append(&mut second_half);
    println!("append colors = {:?}, second half = {:?}", colors, second_half);

    let mut stuff = vec!["1", "2", "3", "4", "5", "6"];
    println!("original stuff {:?}", stuff);
    let stuff_to_insert = vec!["a", "b", "c"];

    let removed_stuff: Vec<_> = stuff.splice(1..4, stuff_to_insert).collect();
    println!("Splice Stuff = {:?}, Removed Stuff = {:?}", stuff, removed_stuff);

    //large vec
    let mut large_vec: Vec<i32> = Vec::with_capacity(1_000_000);
    println!("large_vec len = {}, cap = {}", large_vec.len(), large_vec.capacity());
    large_vec.shrink_to_fit();
    println!("shirink large_vec len = {}, cap = {}", large_vec.len(), large_vec.capacity());

    let mut nums = vec![1, 2, 3, 4];
    let second_num = nums.swap_remove(1);
    println!("nums = {:?}, second_num = {}", nums, second_num);

    println!("hello vec")

}