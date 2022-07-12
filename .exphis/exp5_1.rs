use std::{ cell::RefCell,  collections::{HashMap, hash_map::{Iter}} };
use std::{ fmt::{Display}, ops::AddAssign, rc::Rc, vec };

//字符频率统计
pub struct CharMapper {
    pub mapper : HashMap<char,usize>
}

impl CharMapper {
    pub fn build(input: &String) -> Self {
        let mut map:HashMap<char,usize> = HashMap::new();
        for(_,ch) in  input.char_indices() {
            map.entry(ch).or_insert(0).add_assign(1);
        }
        Self { mapper : map }
    }
    pub fn size(&self) -> usize {
        self.mapper.len()
    }
    pub fn iter(&self) -> Iter<char,usize> {
        self.mapper.iter()
    }
}

type RefHuffmanTree = Rc<RefCell<HuffmanTree>>;

//哈夫曼树结点
pub struct HuffmanTree {
    pub value : Option<char>,
    pub weight : usize,
    pub father : Option<RefHuffmanTree>,
    pub leftson : Option<RefHuffmanTree>,
    pub rightson : Option<RefHuffmanTree>,
}

impl HuffmanTree {
    pub fn new() -> Self {
        Self {
            value : None,
            weight : 0,
            father : None,
            leftson : None,
            rightson : None,
        }
    }

    //构建哈夫曼树
    pub fn build(map: CharMapper) -> RefHuffmanTree {
        let n = map.size();
        let all = (0..2*n-1)
            .map(|_| Rc::new(RefCell::new(Self::new())))
            .collect::<Vec<RefHuffmanTree>>();
        //初始化结点
        map.iter().enumerate().into_iter().for_each(|(index,(ch,wt))| {
            all[index].borrow_mut().value = Some(*ch);
            all[index].borrow_mut().weight = *wt;
        });
        //更新树
        for index in n..2*n-1 {
            // 找到 [0, index-1] 中权重最小的结点
            let m1 = Self::find_min(&all[..index]).unwrap();
            // 标记父结点为 index 上的结点，下次就不会找到这个
            m1.borrow_mut().father = Some(all[index].clone());
            // 找到 [0, index-1] 中权重第二小的结点
            let m2 = Self::find_min(&all[..index]).unwrap();
            // 标记该结点的父结点为 index 上的结点。
            m2.borrow_mut().father = Some(all[index].clone());
            //生成新的权重
            let w1 = m1.as_ref().borrow().weight;
            let w2 = m2.as_ref().borrow().weight;
            let weight = w1 + w2;
            //创建新的结点
            all[index].borrow_mut().weight = weight;
            all[index].borrow_mut().leftson = Some(m1.clone());
            all[index].borrow_mut().rightson = Some(m2.clone());
        }
        all.last().unwrap().clone()
    }

    // 获取最小的值
    pub fn find_min(tree_slice: &[RefHuffmanTree]) -> Option<RefHuffmanTree> {
        let mut min = usize::MAX;
        let mut result = None;
        for tree in tree_slice {
            let tree_cell = tree.as_ref();
            if tree_cell.borrow().father.is_none() && tree_cell.borrow().weight < min {
                min = tree_cell.borrow().weight;
                result = Some(tree.clone());
            }
        }
        result
    }
}

//展示树的结构及其相关信息
pub struct HuffmanTreeInfo {
    pub mapinfo : HashMap<char,Vec<bool>>
}

impl HuffmanTreeInfo {
    pub fn build(huffman_tree: RefHuffmanTree) -> Self {
        let mut map = HashMap::new();
        Self::tree_dfs(&Some(huffman_tree),&mut map,&mut vec![]);
        Self { mapinfo : map }
    }
    pub fn tree_dfs(
        tree : & Option<RefHuffmanTree>,
        map : &mut HashMap<char,Vec<bool>>,
        vec : &mut Vec<bool>,
    ) {
        if let Some(tree) = tree {
            let tree = tree.as_ref().borrow();
            if let Some(ch) = tree.value {
                map.insert(ch, vec.clone());
            }
            vec.push(false);
            Self::tree_dfs(&tree.leftson, map, vec);
            let last = vec.last_mut().unwrap();
            *last = true;
            Self::tree_dfs(&tree.rightson, map, vec);
            vec.pop();
        }
    }
}

//适配到标准输出接口
impl Display for HuffmanTreeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        self.mapinfo.iter()
            .for_each(|(c, vec)| {
                let mut strs = String::new();
                vec.iter().for_each(|b| {
                    strs += if *b { "1" } else { "0" }
                });
                buf += format!("{}:{}\n", *c, strs).as_str();
            });
        f.write_str(buf.as_str())
    }
}

//对指定的字符串构建哈夫曼编码树
fn main(){
    let line = String::from("dlsfkjsldkjgalsk");
    println!("需要解析的字符串是:{}",line);
    let mapper = CharMapper::build(&line);
    println!("字符串出现的频率统计:");
    for (ch,len) in mapper.iter() {
        println!("{} - {}",ch,len);
    }
    //构造哈夫曼树
    let tree = HuffmanTree::build(mapper);
    //获取哈夫曼树的编码信息
    let info = HuffmanTreeInfo::build(tree);
    println!("各个字符的哈夫曼编码:\n{}",info);
}

