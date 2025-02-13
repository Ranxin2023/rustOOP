struct TreeNode{
    value: i32,
    left:Option<Box<TreeNode>>,
    right:Option<Box<TreeNode>>,
}
impl TreeNode{
    fn new(value:i32)->Self{
        TreeNode{
            value, 
            left:None, 
            right:None,
        }
    }
}
struct BST{
    root:Option<Box<TreeNode>>, 
}

impl BST{
    fn new()->Self{
        BST{
            root:None, 
        }
    }
    fn insert(&mut self, value: i32){
        Self::insert_node(&mut self.root, value);
    }
    fn insert_node(node: &mut Option<Box<TreeNode>>, value:i32){
        match node{
            Some(ref mut node)=>{
                if value<node.value{
                    Self::insert_node(&mut node.left, value);
                }
                else {
                    Self::insert_node(&mut node.right, value);
                }
            }
            None=>{
                *node=Some(Box::new(TreeNode::new(value)));
            }
        }
    }
    fn search(&self, value:i32)->bool{
        Self::search_node(&self.root, value)
    }
    fn search_node(node:&Option<Box<TreeNode>>, value:i32)->bool{
        match node{
            Some(ref node)=>{
                if value==node.value{
                    true
                }
                else if value<node.value{
                    return Self::search_node(&node.left, value)
                }
                else{
                    return Self::search_node(&node.right, value)
                }
            }
            None=>false,
        }
    }
}
fn main(){
    let mut bst=BST::new();
    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    
    println!("Searching for node 7:{}", bst.search(7));
    println!("Searching for node 6:{}", bst.search(6));
}