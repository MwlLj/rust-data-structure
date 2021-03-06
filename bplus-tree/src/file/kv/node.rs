use serde_derive::{Deserialize, Serialize};

pub enum Error {
    SerdeError,
    CrossTheBorder
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NodePos {
    pub startPos: usize,
    pub endPos: usize
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DataPos {
    pub fileName: String,
    pub startPos: usize,
    pub endPos: usize
}

impl Default for NodePos {
    fn default() -> Self {
        Self{
            startPos: 0,
            endPos: 0
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LeafPageHeader {
    /*
    ** 如果该页被删除, 这里将保存该页在删除队列中的位置
    */
    pub pre: NodePos,
    pub next: NodePos,
    pub delNext: NodePos,
    /*
    ** items的有效长度 (正在使用的item长度)
    */
    pub itemLen: usize
}

impl Default for LeafPageHeader {
    fn default() -> Self {
        Self{
            pre: NodePos::default(),
            next: NodePos::default(),
            delNext: NodePos::default(),
            itemLen: 0
        }
    }
}

impl LeafPageHeader {
    pub fn oneLen() -> Option<usize> {
        match bincode::serialize(&LeafPageHeader::default()) {
            Ok(c) => {
                Some(c.len())
            },
            Err(err) => {
                None
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LeafItem {
    pub key: Vec<u8>,
    pub value: NodePos
}

impl LeafItem {
    pub fn oneLen(keyMax: usize) -> Option<usize> {
        // let mut k = Vec::with_capacity(keyMax);
        let mut k: Vec<u8> = Vec::new();
        k.resize(keyMax, 0);
        match bincode::serialize(&LeafItem{
            key: k,
            value: NodePos::default()
        }) {
            Ok(c) => {
                Some(c.len())
            },
            Err(err) => {
                None
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LeafNode {
    pub header: LeafPageHeader,
    pub items: Vec<LeafItem>
}

impl LeafNode {
    pub fn set(&mut self, key: &[u8], valuePos: NodePos, pos: usize, keyMax: usize) -> Result<(), Error> {
        match self.items.get_mut(pos) {
            Some(it) => {
                // println!("-------------------------- {}", it.key.len());
                for (i, k) in key.iter().enumerate() {
                    it.key[i] = *k;
                }
                // it.key.copy_from_slice(key);
                it.value = valuePos;
            },
            None => {
                return Err(Error::CrossTheBorder);
            }
        }
        self.header.itemLen += 1;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IndexNode {
    pub keys: Vec<String>,
    pub nodes: Vec<NodePos>
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Node {
    Index(NodePos),
    Leaf(NodePos)
}

impl Default for Node {
    fn default() -> Self {
        Node::Leaf(NodePos::default())
    }
}
