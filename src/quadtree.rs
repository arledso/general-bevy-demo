use core::panic;


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

#[derive(Clone)]
struct BranchChildren<T>{
    nw: Box<Tree<T>>,
    ne: Box<Tree<T>>,
    sw: Box<Tree<T>>,
    se: Box<Tree<T>>,
}

impl<T: Clone> BranchChildren<T>{
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

    fn get_quadrant_bounds(bounds: ((f32,f32),(f32,f32))) -> ((((f32,f32),(f32,f32))), ((f32,f32),(f32,f32)), ((f32,f32),(f32,f32)), ((f32,f32),(f32,f32))) {
        let ((x_1,y_1),(x_2,y_2)) = bounds;
        let (x_o,y_o) = ((x_1+x_2)/2.0, (y_1+y_2)/2.0);

        (((x_1,y_o),(x_o,y_2)), ((x_o,y_o),(x_2,y_2)), ((x_1,y_1),(x_o,y_o)), ((x_o,y_1),(x_2,y_o)))
    }
    //add way to iterate over quadrants
}

impl<T: Clone> Tree<T> {

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
        condition: impl Fn(T, ((f32,f32),(f32,f32))) -> bool,
        bounds: ((f32, f32), (f32, f32))
        )-> Vec<Vec<T>>
        {
        let mut leaf_clusters: Vec<Vec<T>> = Vec::<Vec<T>>::new();

        let mut current_queue: Vec<Box<Tree<T>>> = Vec::<Box<Tree<T>>>::new();
        let mut upcoming_queue: Vec<Box<Tree<T>>> = Vec::<Box<Tree<T>>>::new();
        
        let tree: Box<Tree<T>> = Box::new(Tree::new_cluster(0, bounds, leaves));
        current_queue.push(tree);

        while current_queue.get(0).is_some() {

            for mut tree_node in current_queue {

                let Tree::Cluster{
                    depth: node_depth,
                    bounds: node_bounds,
                    leaves: node_leaves
                } = *tree_node

                else {panic!("PANIC! current tree node leaf and not branch");};
                
                if node_depth == definition || node_leaves.len() <= 1 {
                    leaf_clusters.push(node_leaves);
                    continue;
                }
                
                tree_node = Box::new(Tree::new_branch(node_depth, node_bounds));
                let Tree::Branch {
                    depth: _,
                    bounds: _,
                    children: mut node_children 
                } = *tree_node

                else {panic!("PANIC! current tree node no longer branch")};
                
                upcoming_queue.push(node_children.nw);
                upcoming_queue.push(node_children.ne);
                upcoming_queue.push(node_children.sw);
                upcoming_queue.push(node_children.se);
                
                let nw_bounds = BranchChildren::<T>::get_quadrant_bounds(node_bounds).0;
                let ne_bounds = BranchChildren::<T>::get_quadrant_bounds(node_bounds).1;
                let sw_bounds = BranchChildren::<T>::get_quadrant_bounds(node_bounds).2;
                let se_bounds = BranchChildren::<T>::get_quadrant_bounds(node_bounds).3;
    
                let mut nw_leaves: Vec<T> = Vec::<T>::new();
                let mut ne_leaves: Vec<T> = Vec::<T>::new();
                let mut sw_leaves: Vec<T> = Vec::<T>::new();
                let mut se_leaves: Vec<T> = Vec::<T>::new();

                for leaf in node_leaves {
                    if condition(leaf.clone(), nw_bounds) {nw_leaves.push(leaf.clone());}
                    if condition(leaf.clone(), ne_bounds) {ne_leaves.push(leaf.clone());}
                    if condition(leaf.clone(), sw_bounds) {sw_leaves.push(leaf.clone());}
                    if condition(leaf.clone(), se_bounds) {se_leaves.push(leaf.clone());}
                }
    
                node_children.nw = Box::new(Tree::new_cluster(node_depth + 1, nw_bounds, nw_leaves));
                node_children.ne = Box::new(Tree::new_cluster(node_depth + 1, ne_bounds, ne_leaves));
                node_children.sw = Box::new(Tree::new_cluster(node_depth + 1, sw_bounds, sw_leaves));
                node_children.se = Box::new(Tree::new_cluster(node_depth + 1, se_bounds, se_leaves));
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