extern crate rand;

use std::collections::btree_set::BTreeSet;
use rand::{Rng};

const WIDTH : usize = 39;
const HEIGHT : usize = 62;

struct UnionFindNode {
  parent : usize,
  rank : usize
}

struct UnionFind {
  data : Vec<UnionFindNode>
}

impl UnionFind {
  fn new(size : usize) -> UnionFind {
    let mut my_data = Vec::with_capacity(size);
    for i in 0..size {
      my_data.push(UnionFindNode{parent: i, rank: 0});
    }
    UnionFind{data: my_data}
  }

  fn find(&mut self, index: usize) -> usize {
    let parent = self.data[index].parent;
    if parent != index {
      self.data[index].parent = self.find(parent);
    }
    self.data[index].parent
  }
  
  fn union(&mut self, this: usize, that: usize) {
    let this_root = self.find(this);
    let that_root = self.find(that);
    if this_root == that_root {
      return;
    }
    if self.data[this_root].rank < self.data[that_root].rank {
      self.data[this_root].parent = that_root;
    }
    else if self.data[that_root].rank < self.data[this_root].rank {
      self.data[that_root].parent = this_root  ;
    }
    else {
      self.data[that_root].parent = this_root;
      self.data[that_root].rank += 1;
    }
  }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct MazeCell {
  x : usize,
  y : usize
}

impl MazeCell {
  fn adjacent(&self) -> Vec<MazeCell> {
    let mut border = Vec::new();
    if self.x != 0 {
      border.push(MazeCell{x: self.x-1, y: self.y});
    }
    if self.y != 0 {
      border.push(MazeCell{x: self.x, y: self.y-1});
    }
    border
  }
  
  fn all() -> Vec<MazeCell> {
    let mut cells = Vec::new();
    for x in 0..WIDTH {
      for y in 0..HEIGHT {
        cells.push(MazeCell{x: x, y: y});
      }
    }
    cells
  }

  fn index(&self) -> usize {
    self.x + WIDTH * self.y
  }  
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Border {
  from : MazeCell,
  to   : MazeCell
}

fn main() {
  let mut borders = Vec::new();
  for cell in MazeCell::all() {
    for adj in cell.adjacent() {
      borders.push(Border{from: adj, to: cell.clone()});
    }
  }
  rand::thread_rng().shuffle(borders.as_mut_slice());
  let mut union_find = UnionFind::new(WIDTH * HEIGHT);
  let mut paths = BTreeSet::new();
  for b in borders {
    if union_find.find(b.from.index()) != union_find.find(b.to.index()) {
      union_find.union(b.from.index(), b.to.index());
      paths.insert(b);
    }  
  }
  
  for y in 0..HEIGHT - 1 {
    for x in 0..WIDTH - 1 {
      print!("#");
      if paths.contains(&Border{from : MazeCell{x: x, y: y}, to: MazeCell{x: x+1, y: y}}) {
        print!("#")
      }
      else {
        print!(" ");
      }
    }
    println!("#");
    for x in 0..WIDTH {
      if paths.contains(&Border{from: MazeCell{x: x, y: y}, to: MazeCell{x: x, y: y+1}}) {
        print!("# ");
      }    
      else {
        print!("  ");
      }
    }
    println!("");
  }
  for x in 0..WIDTH - 1 {
    print!("#");
    if paths.contains(&Border{from: MazeCell{x: x, y: HEIGHT-1}, to: MazeCell{x: x+1, y: HEIGHT-1}}) {
      print!("#");
    }
    else {
      print!(" ");
    }
  }
  println!("#");
}