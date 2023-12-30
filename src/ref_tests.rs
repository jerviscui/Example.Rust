pub fn refs_print() {
    let a = A { s: "a".to_string() };
    let r: &A = &a;
    println!("        r   {:p}, &r   {:p}", r, &r);

    // borrow copy to parameter
    A::set_ref(r);
    // borrow copy to new variable
    let r_2 = r;
    A::set_ref(r);
    println!("r_copy  r2  {:p}, &r2  {:p}", r_2, &r_2);

    println!();

    let mut ma = a; // a move to ma, Then use the new addr
    let mr: &mut A = &mut ma;
    // error[E0499]: cannot borrow `ma` as mutable more than once at a time
    // let mr_twice: &mut A = &mut ma;
    println!("        mr  {:p}, &mr  {:p}", mr, &mr); // mr != r

    // immutable borrow also use the new addr
    let x: &A = mr;
    let x: &A = &*mr;
    println!("        x   {:p}, &x   {:p}", x, &x); // x == mr
                                                    // error[E0308]: mismatched types
                                                    // let xx:&mut A = x;

    // mutable borrow copy to parameter
    A::set_mref(mr);
    println!("{:?}", mr);
    // mmutalbe borrow move to new variable
    let mr_2 = mr; // borrow mr moved, addr dose not change
                   // error[E0382]: borrow of moved value: `mr`
                   // A::set_mref(mr);
    println!("mr_move mr2 {:p}, &mr2 {:p}", mr_2, &mr_2);

    // mr out of scope, borrow `ma` as mutable 2th
    let mr = &mut ma;
    println!("mr2th   mr  {:p}, &mr  {:p}", mr, &mr);

    // error[E0507]: cannot move out of `*mr` which is behind a mutable reference
    // let a: A = *mr;
    let r: &A = &*mr;
    // error[E0507]: cannot move out of `*r` which is behind a shared reference
    // let a: A = *r;

    println!();

    println!("           r     {:p}, &r     {:p}", r, &r);
    let ret_r = A::return_ref(r);
    println!("           retr  {:p}, &retr  {:p}", ret_r, &ret_r);

    println!("           mr    {:p}, &mr    {:p}", mr, &mr);
    let ret_mr = A::return_mref(mr);
    println!("           retmr {:p}, &retmr {:p}", ret_mr, &ret_mr);
}

#[derive(Debug)]
struct A {
    s: String,
}

impl A {
    fn set_ref(a: &A) {
        println!("ref()   a   {:p}, &a   {:p}", a, &a);
    }

    fn set_mref(a: &mut A) {
        println!("mref()  a   {:p}, &a   {:p}", a, &a);
        a.s = "abc".to_string();
    }

    fn return_ref(a: &A) -> &A {
        let b = a;
        println!("ret_ref()  b     {:p}, &b     {:p}", b, &b);
        b
    }

    fn return_mref(a: &mut A) -> &mut A {
        let b = a;
        println!("ret_mref() b     {:p}, &b     {:p}", b, &b);
        b
    }
}
