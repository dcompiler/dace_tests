use dace::ast::Node;
use std::rc::Rc;

fn trmm_trace(M: usize, N:usize) -> Rc<Node> {

    let i_loop_ref = Node::new_single_loop("i", 0, M as i32);
    let j_loop_ref = Node::new_single_loop("j", 0, N as i32);
    let k_loop_ref = Node::new_single_loop("k", Node::get_lb(&i_loop_ref).unwrap() + 1, M as i32);

    // B[i * N + j] += A[k * M + i] * B[k * N + j];
    let a_ref = Node::new_ref("A", vec![N, M], |ijk| {
        vec![ijk[2] as usize, ijk[0] as usize]
    });
    let b1_ref = Node::new_ref("B", vec![M, N], |ijk| {
        vec![ijk[2] as usize, ijk[1] as usize]
    });
    let b2_ref = Node::new_ref("B", vec![M, N], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize]
    });

    Node::extend_loop_body(&k_loop_ref, &a_ref);
    Node::extend_loop_body(&k_loop_ref, &b1_ref);
    Node::extend_loop_body(&k_loop_ref, &b2_ref);

    // B[i * N + j] = alpha * B[i * N + j];
    let b3_ref = Node::new_ref("B", vec![M, N], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize]
    });
    Node::extend_loop_body(&j_loop_ref, &b3_ref);
    Node::extend_loop_body(&j_loop_ref, &k_loop_ref);

    Node::extend_loop_body(&i_loop_ref, &j_loop_ref);

    i_loop_ref
}

fn _2mm(NI: usize, NJ: usize, NK: usize, NL: usize) -> Vec<Rc<Node>> {
    let s_ref_tmp = Node::new_ref("tmp", vec![NI, NJ], |ijk| {
        vec![ijk[0] as usize, ijk[1] as usize]
    });
    let s_ref_a = Node::new_ref("a", vec![NI, NK], |ijk| {
        vec![ijk[0] as usize, ijk[2] as usize]
    });
    let s_ref_b = Node::new_ref("b", vec![NK, NJ], |ijk| {
        vec![ijk[2] as usize, ijk[1] as usize]
    });
    let s_ref_c = Node::new_ref("c", vec![NL, NJ], |ijk| {
        vec![ijk[3] as usize, ijk[1] as usize]
    });
    let s_ref_d = Node::new_ref("d", vec![NI, NL], |ijk| {
        vec![ijk[0] as usize, ijk[3] as usize]
    });

    let knk_loop_ref = Node::new_single_loop("k", 0, NK as i32);
    Node::extend_loop_body(&knk_loop_ref, &s_ref_a);
    Node::extend_loop_body(&knk_loop_ref, &s_ref_b);
    Node::extend_loop_body(&knk_loop_ref, &s_ref_tmp);
    
    let jnj_loop_ref = Node::new_single_loop("j", 0, NJ as i32);
    Node::extend_loop_body(&knk_loop_ref, &s_ref_tmp);
    Node::extend_loop_body(&knk_loop_ref, &knk_loop_ref);
    
    let ini_loop_ref1 = Node::new_single_loop("i", 0, NI as i32);
    Node::extend_loop_body(&ini_loop_ref1, &jnj_loop_ref);

    let knj_loop_ref = Node::new_single_loop("k", 0, NJ as i32);
    Node::extend_loop_body(&knj_loop_ref, &s_ref_tmp);
    Node::extend_loop_body(&knj_loop_ref, &s_ref_c);
    Node::extend_loop_body(&knj_loop_ref, &s_ref_d);
    
    let jnl_loop_ref = Node::new_single_loop("j", 0, NL as i32);
    Node::extend_loop_body(&jnj_loop_ref, &s_ref_d);
    Node::extend_loop_body(&jnj_loop_ref, &knj_loop_ref);

    let ini_loop_ref2 = Node::new_single_loop("i", 0, NI as i32);
    Node::extend_loop_body(&ini_loop_ref2, &jnl_loop_ref);


    let mut vec = Vec::new();
    vec.push(ini_loop_ref1);
    vec.push(ini_loop_ref2);

    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    fn trmm_trace_test() {
        let M = 1024;
        let N = 1024;

        let ast = trmm_trace(M, N);
        assert_eq!(ast.node_count(), 7);
    }
    
    fn _2mm_test() {
        let ast = _2mm(1024, 1024, 1024, 1024);
        assert_eq!(ast.len(), 2);
    }

}
