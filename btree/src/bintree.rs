
pub struct BinNode<T> {
    value: T,
    left: Option<Box<BinNode<T>>>,
    right: Option<Box<BinNode<T>>>,
}


pub struct BinTree<T> {
    root: Option<Box<BinNode<T>>>,
}


impl<T> BinTree<T> {
    
}


