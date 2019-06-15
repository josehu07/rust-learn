mod print_hello;    // Seperates the contents of module `print_hello` into another file with the same name.
                    //   Think of this equivalent to defining a `mod print_hello {...}` block right here,
                    //   where `...` is the content of the file `print_hello.rs`.


fn main() {
    print_hello::display_msg();
}
