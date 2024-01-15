pub mod linked {
    enum List {
        Cons(u32, Box<List>),
        Nil,
    }
}
