fn main() {
    //   let v = (let x = 3);
    let v = {
        let x = 3;
        x
    };

    assert!(v == 3);
}
