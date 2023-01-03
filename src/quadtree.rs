
#[derive(Clone)]
enum Tree<T> {
    Branch {
        depth: u32,
        children: BranchChildren<T>,
    },
    Cluster {
        depth: u32,
        leaves: Vec<T>,
    }
}

#[derive(Clone)]
struct BranchChildren<T>{
    nw: Box<Tree<T>>,
    ne: Box<Tree<T>>,
    sw: Box<Tree<T>>,
    se: Box<Tree<T>>,
} 

impl<T> BranchChildren<T>{
    fn new(depth: u32) -> BranchChildren<T> {
        return BranchChildren {
            nw: Box::new(Tree.new_cluster(depth, Vec::<T>::new())),
            ne: Box::new(Tree.new_cluster(depth, Vec::<T>::new())),
            sw: Box::new(Tree.new_cluster(depth, Vec::<T>::new())),
            se: Box::new(Tree.new_cluster(depth, Vec::<T>::new())),
        }
    }
}

impl<T> Tree<T> {

    fn new_cluster (depth: u32, leaves: Vec<T>) -> Tree<T>{
        return Tree::Cluster {depth: depth, leaves: leaves}
    }

    fn new_branch (depth: u32) -> Tree<T>{
        return Tree::Branch {
            depth: depth,
            children: BranchChildren::new(depth + 1)
        }
    }

    fn leaf_clusters<T>(definition: u32, leaves: T) -> Vec<Vec<T>>{
        let mut leaf_clusters: Vec<Vec<T>> = Vec::<Vec<T>>::new();

        let mut current_queue: Vec<Box<Tree<T>>> = Vec::<Box<Tree<T>>>::new();
        let mut upcoming_queue: Vec<Box<Tree<T>>> = Vec::<Box<Tree<T>>>::new();
        
        let mut tree: Box<Tree<T>> = Box::new(Tree::new_cluster(0, leaves));
        current_queue.push(tree);

        while current_queue.get(0).is_some() {

            for mut tree_node in current_queue {
                if tree_node.depth == definition || tree_node.leaves <= 1 {
                    leaf_clusters.push(tree_node.leaves);
                    continue;
                }
                
                let cluster_leaves: Vec<T> = tree_node.leaves.clone();
                let current_depth: u32 = tree_node.depth.copy();
                tree_node = Tree::new_branch(current_depth);
                
                upcoming_queue.push(tree_node.nw);
                upcoming_queue.push(tree_node.ne);
                upcoming_queue.push(tree_node.sw);
                upcoming_queue.push(tree_node.se);
                //add leaves to respective branches
            }
            current_queue = upcoming_queue;
            upcoming_queue = Vec::<Box<Tree<T>>>::new();
        }

        return leaf_clusters;  
    }
}

//start with Cluster node
// if Tree.depth = n 
// => leaves.push(cluster), continue
// if cluster.size > 1
// => copy cluster to local var, let cur_node = Branch
// populate_branches(condition)
// add new branches to next queue
// move along cur_queue