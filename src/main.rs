use std::rc::Rc;

mod list;

struct Inner {
    v1: i32,
    v2: f64,
    v3: i64,
}

struct Tmp {
    val: Inner,
}

fn main() {
    let a = Tmp {
        val: Inner {
            v1: 12,
            v2: 12.0,
            v3: 12,
        }
    };
    println!("{}", (&a as *const Tmp) as usize);
    print(&a);
}

fn print(tmp: &Tmp) {
    println!("{}", (tmp as *const Tmp) as usize);
}
