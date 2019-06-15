fn main() {
    
    // Mutability.
    let x = 2;
    let mut y = 3;
    y = 1 - 4 * y;
    println!("{} {}", x, y);

    // Constants.
    const CST : u32 = 12;
    println!("{}", CST);

    // Shadowing.
    let chars = "hahahaha";     // NOTICE: string literals are slices.
    {
        let chars = chars.len();
        println!("{}", chars);
    }
    println!("{}", chars);

    // Statically typed!
    //   u/i 8 16 32 64 128 size
    //   f 32 64
    //   bool
    //   char (4 bytes in size, Unicode scalar value)
    let num : f32 = "3.14".parse().expect("Not a float!");
    let inum = match 17 {
        0 => 7,
        1 => 2_300,
        2 => 0xff,
        3 => 0o53,
        _ => 0b1100_1011 % 13,
    };
    let u8num = b'A';
    let heart_eyed_cat = '😻';
    println!("{} {} {} {}", num, inum, u8num, heart_eyed_cat);

    // Tuples.
    let tup1 = (3, 'D', "M", 5.12);
    let tup2 : (char, i32, f64) = ('Z', 5, 9.12);
    println!("{}", tup1.0);
    let (z_char, five, nine_twelve) = tup2;
    println!("{} {} {}", z_char, five, nine_twelve);

    // Arrays.
    let months = ["January", "February", "March"];
    let arr1 : [i32 ; 5] = [1, 2, 3, 4, 5];
    let arr2 = [7 ; 9];
    println!("{}", months[1]);
    println!("{} {}", arr1.len(), arr2.len());

    // Underscored names.
    let _ = 3;      // Single underscore DROPs the binding immediately after this statement.
    let _x = 7;     // Underscored names also do not trigger unused warning,
                    //   but are deconstructed when they go out of scope.
}
