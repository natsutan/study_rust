extern crate rand;

use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;


struct Net {
    nodes:Vec<u32>,
    weight:u32,
}


fn make_netlist() -> Vec<Net> {
    let mut netlist :Vec<Net> = Vec::new();

// for debug
//    let n1 = Net{nodes:vec![1, 2, 3, 4, 5], weight: 1};
//    let n2 = Net{nodes:vec![6, 7, 8, 9, 10], weight: 1};
//    let n1 = Net{nodes:vec![1,2,3,4,5,6], weight: 1};
//    let n2 = Net{nodes:vec![9], weight: 1};

    let n1 = Net{nodes:vec![1, 2, 4, 5], weight: 1};
    let n2 = Net{nodes:vec![2, 3, 5], weight: 1};
    let n3 = Net{nodes:vec![3, 6, 10, 4], weight: 2};
    let n4 = Net{nodes:vec![4, 8, 3, 7], weight: 1};
    let n5 = Net{nodes:vec![5, 7, 1, 6], weight: 3};
    let n6 = Net{nodes:vec![6, 4, 7, 2], weight: 3};
    let n7 = Net{nodes:vec![7, 9, 5], weight: 2};
    let n8 = Net{nodes:vec![8, 2], weight: 3};
    let n9 = Net{nodes:vec![9, 10,  5], weight: 2};
    let n10 = Net{nodes:vec![10, 5], weight: 4};

    netlist.push(n1);
    netlist.push(n2);
    netlist.push(n3);
    netlist.push(n4);
    netlist.push(n5);
    netlist.push(n6);
    netlist.push(n7);
    netlist.push(n8);
    netlist.push(n9);
    netlist.push(n10);

    netlist
}

fn initial_block() -> [u32; 10] {
    let mut rng = thread_rng();
    let mut block:[u32;10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    block.shuffle(&mut rng);
    block
}

fn same_area(block:&[u32;10], net:&Net) -> bool {
    let block_a = &block[0..5];
    let block_b = &block[5..10];
    //全てtrue、もしくは全てfalseの時trueを返す
    let contain_as: Vec<bool> = block_a.into_iter().map(|x| net.nodes.contains(x)).collect();
    let contain_bs: Vec<bool> = block_b.into_iter().map(|x| net.nodes.contains(x)).collect();

//    println!("as_ {:?}", contain_as);
//    println!("bs_ {:?}", contain_bs);
    let all_b = !contain_as.into_iter().fold(false, |all_a, x| all_a | x);
    let all_a = !contain_bs.into_iter().fold(false, |all_b, x| all_b | x);

//    println!("{} {}", all_a, all_b);

    all_a || all_b
}


fn cost(block:&[u32;10], netlist:&Vec<Net>) -> u32 {
    // blockの前半が、分割したエリアに全て入ればそのまま、そうで無ければそのネットに対応したweightを加算する。
    netlist.into_iter().fold(0,|cost, net| if same_area(&block, net) {cost} else {cost + net.weight} )
}

fn swap_node(block:&[u32;10]) -> [u32;10] {
    let mut new_block = block.clone();
    let mut rng = thread_rng();

    let choice_a = rng.gen_range(0, 6);
    let choice_b = rng.gen_range(6, 10);

    new_block.swap(choice_a, choice_b);
    new_block
}

#[test]
fn cost_test(){
    let netlist = make_netlist();
    let mut block:[u32; 10] = [2, 4, 6, 7, 8, 1, 3, 5, 9, 10];
    let cur_cost = cost(&block, &netlist);
    assert_eq!(cur_cost, 10)
}



fn simulated_annealing(s0:u32, best_s:u32, t:f32, alpha:f32, beta:f32, max_time:f32, m:f32, netlist:&Vec<Net> ) -> [u32;10]
{
    //let mut block:[u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut block = initial_block();

    let cur_cost = cost(&block, &netlist);
    println!("cost = {}　block_a = {:?} block_b = {:?} ", cur_cost,  &block[0..5], &block[5..10]);

    let new_block = swap_node(&block);
    println!("swap　block_a = {:?} block_b = {:?} ",  &new_block[0..5], &new_block[5..10]);


    new_block

}


fn main() {
    let netlist = make_netlist();

    let T0 = 10.0;
    let alpha = 0.9;
    let beta = 1.0;
    let m = 10.0;

    let best_block = simulated_annealing(0, 0, T0, alpha, beta, 0.0, 0.0, &netlist);
    let best_cost = cost(&best_block, &netlist);

    println!("cost = {}　block_a = {:?} block_b = {:?} ", best_cost,  &best_block[0..5], &best_block[5..10]);

}
