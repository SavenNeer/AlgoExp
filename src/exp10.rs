// main.rs

use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq)]
struct Part {
    //部件的相关信息
    weight : usize,
    cost : usize,
    level : usize,
    num : usize,
    value : [usize;99],
}

impl Part { 
    pub fn new(w:usize,c:usize,l:usize,n:usize) -> Self {
        Self {
            weight : w,
            cost : c,
            level : l,
            num : n,
            value : [0usize;99],
        }
    }
}

//Part自定义排序接口
impl Ord for Part {
    fn cmp(&self,rhs:&Self) -> Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

impl PartialEq for Part {
    fn eq(&self,rhs:&Self) -> bool {
        return self.weight == rhs.weight &&
                self.level == rhs.level &&
                self.num == rhs.num;
    }
}

impl PartialOrd for Part {
    //排序策略
    fn partial_cmp(&self,rhs:&Self) -> Option<Ordering> {
        if self.eq(rhs) {
            return Some(Ordering::Equal);
        }
        //Greater表示优先级更高
        if self.weight != rhs.weight {
            if self.weight > rhs.weight {
                return Some(Ordering::Greater);
            }
        } else if self.level != rhs.level {
            if self.level > rhs.level {
                return Some(Ordering::Greater);
            }
        } else {
            if self.num > rhs.num {
                return Some(Ordering::Greater);
            }
        }
        return Some(Ordering::Less);
    }
}

//主函数
#[allow(non_snake_case)]
fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let lis = line.split_whitespace()
            .map(|ch| ch.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
    let n = lis[0];//部件的个数
    let m = lis[1];//供应商个数
    let d = lis[2];//总价上限
    //读取价格
    let mut c:Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        line = "".to_string();
        std::io::stdin().read_line(&mut line).unwrap();
        c.push(line.split_whitespace()
                .map(|ch|ch.parse::<usize>().unwrap())
                .collect::<Vec<usize>>());
    }
    //部件重量
    let mut w:Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        line = "".to_string();
        std::io::stdin().read_line(&mut line).unwrap();
        w.push(line.split_whitespace()
                .map(|ch|ch.parse::<usize>().unwrap())
                .collect::<Vec<usize>>());
    }
    //优先队列初始化
    let mut que:BinaryHeap<Part> = BinaryHeap::new();
    for i in 0..m {
        let mut t = Part::new(w[0][i],c[0][i],0,i);
        if t.cost <= d {
            t.value[0] = i + 1;
            que.push(t);
        }
    }
    //分支界限法
    let mut minW = 99999;
    let mut ans = [0usize;999];
    while !que.is_empty() {
        let temp = que.pop().unwrap();
        if temp.level < n - 1 {
            for i in 0..m {
                let mut t = Part::new(temp.weight+w[temp.level+1][i],
                                        temp.cost+c[temp.level+1][i],
                                        temp.level+1,i);
                if t.level == n - 1 {
                    if t.weight < minW && t.cost <= d {
                        //剪枝
                        minW = t.weight;
                        for i in 0..n-1 {
                            ans[i] = temp.value[i];
                        }
                        ans[n-1] = i + 1;
                    }
                }
                if t.level < n - 1 {
                    if t.cost <= d {
                        //剪枝
                        for i in 0..t.level {
                            t.value[i] = temp.value[i];
                        }
                        t.value[t.level] = i + 1;
                        que.push(t);
                    }
                }
            }
        }
    }
    println!("最小机器重量:{}",minW);
    println!("不同部件的供应商编号:{:?}",&ans[..n]);
}

