use toy_vec::ToyVec;

fn main(){
    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());
    v.push("Budgerigar".to_string());

    let mut iter = v.iter();

    assert_eq!(iter.next(), Some(&"Java Finch".to_string()));
    v.push("Canary".to_string());

    // ここでiter.next()を実行しようとすると、iterはここまで生存していることになる。
    // iterが生存している間はiterがvの不変参照を保持しているため、
    // vの可変参照を引数とする↑のv.push()はコンパイルエラーとなる。
    // assert_eq!(iter.next(), Some(&"Budgerigar".to_string()));
}