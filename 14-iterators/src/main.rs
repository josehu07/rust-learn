fn main() {
    
    // Simple iterator.
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for item in v1_iter {
        print!("{} ", item);
    }
    println!("");

    // An iterator has the trait `Iterator`, which must have the `next()` method.
    let v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter();
    println!("{:?} {:?} {:?} {:?}", v2_iter.next(), v2_iter.next(), v2_iter.next(), v2_iter.next());

    // We have immutable-ref iterators, owned iterators, and mutable-ref iterators.
    let v3 = vec!["AAA".to_string(), "BBB".to_string(), "CCC".to_string()];
    let v4 = vec!["AAA".to_string(), "BBB".to_string(), "CCC".to_string()];
    let mut v5 = vec!["AAA".to_string(), "BBB".to_string(), "CCC".to_string()];
    for _item in v3.iter() {}
    for _item in v4.into_iter() {}
    for item in v5.iter_mut() {
        item.make_ascii_lowercase();
    }
    println!("{:?} {:?}", v3, v5);  // `v4` is already moved.

    // Consuming adaptors (folding): methods that use up an iterator, e.g. `sum()`.
    let v6 = vec![1, 2, 3];
    println!("{}", v6.iter().sum::<i32>());

    // Iterator adaptors (mapping): methods that produce another iterator, e.g. `map()` / `filter()`.
    let v7 = vec![1, 2, 3, 4, 5];
    let boundary = 3;
    println!("{:?}", v7.iter().map(|x| x + 1).collect::<Vec<i32>>());
    println!("{:?}", v7.into_iter().filter(|x| x > &boundary).collect::<Vec<i32>>());

    // Provide a `next()` method to enable self-defined iterators.
    let counter1 = Counter { value : 0 };
    let counter2 = Counter { value : 0 };
    println!("{:?}", counter1.zip(counter2.skip(2))
                             .map(|(x, y)| x + y)
                             .filter(|x| x % 4 == 0)
                             .sum::<u32>());
}


// Self defined iterators, deriving from `Iterator` trait.
#[derive(Debug)]
struct Counter {
    value : u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.value += 1;
        if self.value < 7 {
            Some(self.value)
        } else {
            None
        }
    }
}
