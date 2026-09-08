// --- Core Properties ---

// 1. All leaf nodes must be at the same level (depth) [00:04:06].
//    This is the key property that keeps the tree balanced.

// 2. Every node has a maximum number of keys, let's call it 'M'.
//    (In the video's example, M=4) [00:02:41].

// 3. Every node (except the root) must have a minimum number of keys.
//    This minimum is half of the maximum, rounded down (e.g., M/2) [00:04:26].

// 4. The root node is an exception: it can have fewer than the
//    minimum number of keys (e.g., it can have just one) [00:04:47].

// 5. A non-leaf node that has 'k' keys must have exactly 'k+1' children [00:03:00].
//    Keys in a node act as separators for the subtrees pointed to by its children.

// --- Deletion Rules (Removing a Key) ---

// 10. If a key is deleted from a node and that node *still* meets
//     the minimum key requirement, you are done [00:07:56].

// 11. If deleting a key causes a node to fall *below* the minimum
//     number of keys [00:08:17]:
//     a. (Rotation) First, check its immediate left or right sibling.
//        If a sibling has *more* than the minimum keys, "borrow" a
//        key from it [00:08:31].
//     b. This "rotation" involves moving the separator key from the
//        parent down into the underfull node, and moving a key
//        from the "rich" sibling up to replace the parent's
//        separator [00:09:19].

// 12. (Merging) If *both* adjacent siblings are also at the minimum
//     key count and cannot spare a key [00:09:44]:
//     a. The underfull node, one of its minimum-sized siblings,
//        and the separator key from the parent *all merge* into
//        a single, new node [00:09:58, 00:10:05].
//     b. This removes a key from the parent node.

// 13. This "merging" process is recursive [00:10:13]. If removing the
//     separator from the parent (in rule 12) causes the *parent*
//     to become underfull, you must re-apply the deletion
//     rules (rotation or merging) to the parent [00:10:18].

// 14. (Special Case) If you delete a key from an *internal*
//     (non-leaf) node [00:10:31]:
//     a. You must replace it with a new separator key to maintain
//        the tree structure [00:10:37].
//     b. The replacement key is *either* the largest key from the
//        left child's subtree *or* the smallest key from the
//        right child's subtree [00:11:00].
//     c. Deleting this replacement key from its original leaf node
//        might cause *that* leaf to become underfull, triggering
//        the rotation/merging rules (11-13) [00:11:14].

const MAX_KEYS: usize = 2;

#[derive(Debug, Default)]
pub struct BNode
{
    keys: [u32; MAX_KEYS],
    children: [Box<BNode>; MAX_KEYS],
}

pub struct BTree
{
    root: Box<BNode>,
}

impl BTree
{
    pub fn insert(val: u32) {

    }

    pub fn new(arr: &Vec<u32>) -> BTree {
    
   	todo!("impliment new Btree func")
    
    }
}
