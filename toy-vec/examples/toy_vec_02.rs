use toy_vec::ToyVec;

fn main() {
    let _e: Option<&String>;
    {
        let mut v = ToyVec::new();
        v.push("Java Finch".to_string());
        v.push("Budgerigar".to_string());
        let _e = v.get(1);
    }
    assert_eq!(_e, Some(&"Budgerigar".to_string()));
}