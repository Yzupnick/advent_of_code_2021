use std::io;
use std::io::BufRead;


fn main() {
    let (count, _) = 
        io::stdin().lock().lines()
                          .map(|line| readline_or_panic(line) )
                          .map(|input| parse_or_panic(input))
                          .triple_window()
                          .map(|(j,k,l)| {
                                println!("{} {} {}", j, k, l);
                                j + k + l
                          })
                          .fold((0,0), |acc, n | counter(acc, n));
    println!("{}",count - 1);
}


fn counter (state: (i32, i32), next:i32) -> (i32, i32) {
    let (count, last) = state;
    if next > last {
        return (count + 1, next);
    }
    else {
        return (count, next);
    }
}

fn parse_or_panic(input: String) -> i32 {
    match input.parse::<i32>() {
        Ok(n) => {
            n
        }
        Err(_) => {
            println!("Input not a number: {}", input);
            std::process::exit(1);
        }
    }

}

fn readline_or_panic(line:Result<String, std::io::Error>) -> String {
    match line {
        Ok(input) => {
            input
        }
        Err(_) => {
            println!("Error reading from stdin");
            std::process::exit(1);
        }
    }
}

struct TripleWindow<I, T>
where I: Iterator, T: Clone,
{
    iterator: I,
    first_item: Option<T>,
    second_item: Option<T>,
    
}

impl <I, T> Iterator for TripleWindow<I, T>
where I: Iterator<Item = T>, T: Clone + std::fmt::Debug,
{
    type Item = (T, T, T);

    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> { 
        let next_item = self.iterator.next();
        match next_item {
            None => {
                return None;
            }
            Some(k) => {
                match (self.first_item.clone(), self.second_item.clone()) {
                    (None, None) => {
                        self.first_item = self.iterator.next();
                        self.second_item = self.iterator.next();
                        match (self.first_item.clone(), self.second_item.clone()) {
                            (None, _) | (_, None) => {
                                None
                            }
                            (Some(n) , Some(m)) => {
                                Some((k, n,m))
                            }
                        }
                    }
                    (Some(n), Some(m)) => {
                        self.first_item = Some(m.clone());
                        self.second_item = Some(k.clone());
                        return Some( (n,m,k))
                    }
                    (_, _) => {
                        println!("Something went wrong");
                        std::process::exit(1);
                    }
                }

            }
        }
    }
}

trait Window: Iterator {
    fn triple_window(self) -> TripleWindow<Self, Self::Item>
        where Self::Item: Clone, Self: Sized
        {
            return TripleWindow{
                iterator: self,
                first_item: None,
                second_item: None
            }
        }
}

impl<I: Iterator> Window for I {}
