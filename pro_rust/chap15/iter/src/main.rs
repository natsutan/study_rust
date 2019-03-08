
fn triagnle(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n+1 {
        sum += i;
    }
    sum
}


fn triangle2(n: i32) -> i32 {
    (1..n+1).fold(0, |sum, item| sum + item)
}

// イテレータはIteratorを実装した任意の型
// イテレート可能は、IntoIteratorを実装した任意の型。この型に対して、into_iterを呼び出しイテレータ
// を取得できる。ベクターへの参照&vはイテレート可能
// イテレータは値を生成する。その値をItemと言う。
// イテレータを受け取るコードをcosumerと呼ぶ
#[derive(Debug, Copy, Clone)]
struct i32r {
    start: i32,
    end: i32
}

impl i32r {
    pub fn set_start(&mut self, s:i32) {
        self.start = s;
    }
}


impl Iterator for i32r {
    type Item = i32;
    fn next(& mut self) -> Option<i32> {
        if self.start >= self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}

#[derive(Debug)]
struct MyVec {
    v:Vec<i32r>,
    idx:usize
}

impl MyVec {
    pub fn new() -> MyVec {
        let mut my_vec = MyVec {v:vec![], idx:0};
        for i in 1..5 {
            let i32 = i32r{start:0, end:i};
            my_vec.v.push(i32);
        }
        my_vec
    }
}

impl Iterator for MyVec {
    type Item = i32r;
    fn next(&mut self) -> Option<i32r> {
        if self.v.len() == self.idx {
            return None;
        }
        let elm = self.v[self.idx].clone();
        self.idx+=1;
        return Some(elm);
    }
}

//https://stackoverflow.com/questions/30218886/how-to-implement-iterator-and-intoiterator-for-a-simple-struct
impl<'a> IntoIterator for &'a MyVec {
    type Item = i32r;
    type IntoIter = MyVecIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MyVecIterator {
            MyVec:self, index:0
        }
    }
}

struct MyVecIterator<'a> {
    MyVec: &'a MyVec,
    index: usize,
}

impl<'a> Iterator for MyVecIterator<'a> {
    type Item = i32r;
    fn next(&mut self) -> Option<i32r> {
        if self.MyVec.v.len() == self.index {
            return None;
        }
        let elm = self.MyVec.v[self.index];
        self.index+=1;
        return Some(elm);
    }
}


fn main() {
    println!("Hello, world!");

    let mut v = MyVec::new();

    println!("{:?}", v);
    for mut e in v {
        e.set_start(5);
        println!("{:?}", e)
    }
    println!("");
    println!("{:?}", v);


    for e in v {
        println!("{:?}", e)
    }

}
