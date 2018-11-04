extern crate rand;

use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;


struct Net {
    nodes:Vec<u32>,
    weight:u32,
}

type Block =  [u32; 10];

//ネットリストの定義
fn make_netlist() -> Vec<Net> {
    let mut netlist :Vec<Net> = Vec::new();

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

//ランダムにブロックを振り分ける
fn initial_block() -> Block {
    let mut rng = thread_rng();
    let mut new_block: Block = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    new_block.shuffle(&mut rng);
    new_block
}

//引数のnetが、ABどちらかのブロックに入っていればtrue
//AB両方のブロックにまたがっていればfalse
fn same_area(block:&Block, net:&Net) -> bool {
    let block_a = &block[0..5];
    let block_b = &block[5..10];
    //全てtrue、もしくは全てfalseの時trueを返す
    let contain_as: Vec<bool> = block_a.into_iter().map(|x| net.nodes.contains(x)).collect();
    let contain_bs: Vec<bool> = block_b.into_iter().map(|x| net.nodes.contains(x)).collect();

    let all_b = !contain_as.into_iter().fold(false, |all_a, x| all_a | x);
    let all_a = !contain_bs.into_iter().fold(false, |all_b, x| all_b | x);

    all_a || all_b
}

//分割後のcost計算
fn cost(block:&Block, netlist:&Vec<Net>) -> u32 {
    // blockの前半が、分割したエリアに全て入ればそのまま、そうで無ければそのネットに対応したweightを加算する。
    netlist.into_iter().fold(0,|cost, net| if same_area(&block, net) {cost} else {cost + net.weight} )
}

//ランダムに2つのノードを入れ替える
fn swap_node(block:&Block) -> Block {
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

//メトロポリス基準
// random < exp(- d_cost / t)
fn random(delta_cost:i32, t:f32) -> bool {
    let mut rng = thread_rng();
    let rand_value = rng.gen_range(0.0, 1.0);
    let d_cost:f64 = delta_cost as f64;

    rand_value < std::f64::consts::E.powf(- d_cost / t as f64)
}


fn simulated_annealing(mut t:f32, alpha:f32, beta:f32, max_time:f32, mut m:f32 , netlist:&Vec<Net> ) -> Block
{
    let mut cur_s :Block = initial_block();
    let mut time:f32 = 0.0;
    let mut best_s: Block = cur_s.clone();

    while {
        let result = metropolis(cur_s, best_s, t, m, netlist);
        cur_s = result[0];
        best_s = result[1];

        time = time + m * beta;
        t = alpha * t;
        m = m * beta;
        time < max_time
    } {}
    best_s
}

fn metropolis(mut cur_s: Block, mut best_s: Block, t:f32, m:f32, netlist:&Vec<Net>) ->[Block;2]
{
    let cur_cost = cost(&cur_s, netlist);
    let mut best_cost = cost(&best_s, netlist);


    for _x in 0 .. m as u32 {
        //ノードを一つランダムに交換
        let new_s = swap_node(&cur_s);
        let new_cost = cost(&new_s, netlist);
        let delta_cost = new_cost as i32 - cur_cost as i32;

        println!("new_cost = {}　best_cost = {} block_a = {:?} block_b = {:?}", new_cost, best_cost,  &new_s[0..5], &new_s[5..10]);

        if delta_cost < 0 {
            cur_s = new_s;
            if new_cost < best_cost {
                best_s = new_s;
                best_cost = cost(&best_s, netlist);
            }
        } else {
            if random(delta_cost, t) {
                cur_s = new_s;
            }
        }
    }
    [cur_s, best_s]
}



fn main() {
    let netlist = make_netlist();

    let t0 = 10.0;
    let alpha = 0.9;
    let beta = 1.0;
    let m = 10.0;

    let best_block = simulated_annealing(t0, alpha, beta, 100.0, m, &netlist);
    let best_cost = cost(&best_block, &netlist);

    println!("cost = {}　block_a = {:?} block_b = {:?} ", best_cost,  &best_block[0..5], &best_block[5..10]);

}
