use dace::ast::Node;
use std::rc::Rc;

// DACE code for PolyBench goes here.
pub fn lu(n: usize) -> Rc<Node>{
    let ubound = n as i32;
    let ref_a_ij = Node::new_ref("A", vec![n, n], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize]
    });
    let ref_a_ik = Node::new_ref("A", vec![n, n], |ijk| {
        vec![ijk[0] as usize, ijk[2] as usize]
    });
    let ref_a_kj = Node::new_ref("A", vec![n, n], |ijk| {
        vec![ijk[2] as usize, ijk[1] as usize]
    });
    let ref_a_jj = Node::new_ref("A", vec![n, n], |ijk| {
        vec![ijk[1] as usize, ijk[1] as usize]
    });

    let k_loop_ref_j = Node::loop_node!("k", 0 => move |ijk| ijk[1]);
    Node::extend_loop_body(&k_loop_ref_j, &ref_a_ik);
    Node::extend_loop_body(&k_loop_ref_j, &ref_a_kj);
    Node::extend_loop_body(&k_loop_ref_j, &ref_a_ij);

    let j_loop_lower_ref = Node::loop_node!("j", 0 => move |ijk| ijk[0]);
    Node::extend_loop_body(&j_loop_lower_ref, &k_loop_ref_j);
    Node::extend_loop_body(&j_loop_lower_ref, &ref_a_jj);
    Node::extend_loop_body(&j_loop_lower_ref, &ref_a_ij);

    let k_loop_ref_i = Node::loop_node!("k", 0 => move |ijk| ijk[0]);
    Node::extend_loop_body(&k_loop_ref_i, &ref_a_ik);
    Node::extend_loop_body(&k_loop_ref_i, &ref_a_kj);
    Node::extend_loop_body(&k_loop_ref_i, &ref_a_ij);

    let j_loop_upper_ref = Node::loop_node!("j", move |ijk| ijk[0] => ubound);
    Node::extend_loop_body(&j_loop_upper_ref, &k_loop_ref_i);

    let i_loop_ref = Node::new_single_loop("i", 0, ubound);
    Node::extend_loop_body(&i_loop_ref, &j_loop_lower_ref);
    Node::extend_loop_body(&i_loop_ref, &j_loop_upper_ref);
}


#[cfg(test)]
mod tests {
    use super::*;
    
    // Write your unit tests
    #[test]
    fn lu_test() {
        let mm = lu(100);
        assert_eq!(mm.node_count(), 6);
    }
}
