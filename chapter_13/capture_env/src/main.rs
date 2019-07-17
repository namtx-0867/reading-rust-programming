fn main() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
    
    // x is captured

    let v = vec![1, 2, 3];
    let equal_to_v = move |z| z == v;

    // println!("can't use v here!: {:?}", v);
    let t = vec![1, 2, 3];
    assert!(equal_to_v(t));
}

#[cfg(test)]
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

#[test]
fn iterator_map() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
