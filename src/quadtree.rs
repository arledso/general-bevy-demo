
#[derive(Clone)]
enum Tree<T> {
    Branch {
        depth: u32,
        bounds: ((f32, f32), (f32, f32)),
        children: BranchChildren<T>,
    },
    Cluster {
        depth: u32,
        bounds: ((f32, f32), (f32, f32)),
        leaves: Vec<T>,
    }
}

enum Quadrant {
    NW,
    NE,
    SW,
    SE
}

#[derive(Clone)]
struct BranchChildren<T>{
    nw: Box<Tree<T>>,
    ne: Box<Tree<T>>,
    sw: Box<Tree<T>>,
    se: Box<Tree<T>>,
} 

impl<T> BranchChildren<T>{
    fn new(depth: u32, bounds: ((f32,f32),(f32,f32))) -> BranchChildren<T> {
        let ((x_1,y_1),(x_2,y_2)) = bounds;
        let (x_o,y_o) = ((x_1+x_2)/2.0, (y_1+y_2)/2.0); //both used to div into quadrants

        //draw diagram for help understanding
        return BranchChildren {
            nw: Box::new(Tree::new_cluster(depth, ((x_1,y_o),(x_o,y_2)), Vec::<T>::new())),
            ne: Box::new(Tree::new_cluster(depth, ((x_o,y_o),(x_2,y_2)), Vec::<T>::new())),
            sw: Box::new(Tree::new_cluster(depth, ((x_1,y_1),(x_o,y_o)), Vec::<T>::new())),
            se: Box::new(Tree::new_cluster(depth, ((x_o,y_1),(x_2,y_o)), Vec::<T>::new())),
        }
    }
    //add way to iterate over quadrants
}

impl<T> Tree<T> {

    fn new_cluster (depth: u32, bounds: ((f32,f32),(f32,f32)), leaves: Vec<T>) -> Tree<T>{
        return Tree::Cluster {depth: depth, bounds: bounds, leaves: leaves}
    }

    fn new_branch (depth: u32, bounds: ((f32,f32),(f32,f32))) -> Tree<T>{
        return Tree::Branch {
            depth: depth,
            bounds: bounds,
            children: BranchChildren::new(depth + 1, bounds)
        }
    }

    fn leaf_clusters<F>(
        definition: u32,
        leaves: Vec<T>,
        condition: impl Fn(T) -> (Quadrant, T), //change to take in T and bounds/relevant info
        bounds: ((f32, f32), (f32, f32))
        )-> Vec<Vec<T>>
        {
        let mut leaf_clusters: Vec<Vec<T>> = Vec::<Vec<T>>::new();

        let mut current_queue: Vec<Box<Tree<T>>> = Vec::<Box<Tree<T>>>::new();
        let mut upcoming_queue: Vec<Box<Tree<T>>> = Vec::<Box<Tree<T>>>::new();
        
        let mut tree: Box<Tree<T>> = Box::new(Tree::new_cluster(0, bounds, leaves));
        current_queue.push(tree);

        while current_queue.get(0).is_some() {

            for mut tree_node in current_queue {

                let Tree::Cluster{
                    depth: node_depth,
                    bounds: node_bounds,
                    leaves: node_leaves
                } = *tree_node;

                if node_depth == definition || node_leaves.len() <= 1 {
                    leaf_clusters.push(node_leaves);
                    continue;
                }
                
                tree_node = Box::new(Tree::new_branch(node_depth, node_bounds));
                let Tree::Branch {
                    depth: _,
                    bounds: _,
                    children: node_children 
                } = *tree_node;
                
                upcoming_queue.push(node_children.nw);
                upcoming_queue.push(node_children.ne);
                upcoming_queue.push(node_children.sw);
                upcoming_queue.push(node_children.se);

                let Tree::Cluster { depth: _, bounds: _, leaves: nw_leaves} = *node_children.nw;
                let Tree::Cluster { depth: _, bounds: _, leaves: ne_leaves} = *node_children.ne;
                let Tree::Cluster { depth: _, bounds: _, leaves: sw_leaves} = *node_children.sw;
                let Tree::Cluster { depth: _, bounds: _, leaves: se_leaves} = *node_children.se;

                for leaf in node_leaves {
                    match condition(leaf).0 {
                        NW => nw_leaves.push(leaf), //copy cur vec to local var; let nw_node = new_cluster(vec.push(leaf))
                    }
                }
                //add leaves to respective branches
                //use condition()        
                //add way to know quadrant bounds/position
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