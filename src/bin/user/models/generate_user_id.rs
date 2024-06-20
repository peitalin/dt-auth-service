use nanoid::nanoid;


pub fn generate_nano_user_id() -> String {
    let alphabet: [char; 30] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'k',
        'm', 'n', 'p', 'q', 'r', 's', 't',
        // 'u', // leave out u for user prefix
        'v', 'w', 'x', 'z', 'y',
    ];
    // remove 'i', 'j', 'o', '0' from alphabet

    let id = nanoid!(12, &alphabet); //=> "4f90d13a4222"
    // "u4f9d13a42ftr"
    // let id = nanoid::simple(); //=> "Uakgb_J5m9g~0JDMbcJqLJ"
    id
}


#[test]
fn generates_nano_user_id_well() {
    let id = generate_nano_user_id();
    assert_eq!(
        id.len(),
        (12 as usize)
    );
}