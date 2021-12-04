use std::io;
use std::io::BufRead;


fn main() {
    let diagnostic  = 
        io::stdin().lock().lines()
                          .map(readline_or_panic)
                          .map(parse_point_or_panic)
                          .fold(Diagnosticts::new(), index_diagnostic_data);
    println!("{}",diagnostic.power_comsumption());
}

// state is stored as an array of BitCounts, index in array is equal to the power of the column.
type IndexedDiagnosticData = [BitCount; usize::BITS as usize];

struct Diagnosticts {
    data: IndexedDiagnosticData,
    size: Option<usize>,
}

impl Diagnosticts {
    fn new() -> Diagnosticts {
        Diagnosticts {
            data: [BitCount::new(); usize::BITS as usize],
            size: None
        }
    }
    fn most_common_bits(&self) -> usize {
        let mut current = 0;
        for (power, count) in self.data.iter().enumerate() {
            if count.ones > count.zeros {
                let cmp = usize::pow(2, power as u32);
                current = current | cmp;
            }
        }
        return current;
    }

    fn gamma_rate(&self) -> usize {
        return self.most_common_bits();
    }

    fn epsilon_rate(&self) -> usize {
        let common_bits = self.most_common_bits();
        println!("{}",common_bits);
        if self.size.is_none() {
            println!("Something went wrong, diagnostic size is set to None");
            std::process::exit(1);
        } else {
            let size = self.size.unwrap();
            let inverted = size ^ usize::MAX;
            let mask = inverted >> (usize::BITS - (size as u32));
            return common_bits ^ mask;
        }
    }

    fn power_comsumption(&self) -> usize {
        self.gamma_rate() * self.epsilon_rate()
    }

}

struct DiagnosticDataPoint {
    value: usize,
    size: usize
}

#[derive(Default, Clone, Copy, Debug)]
struct BitCount {
    zeros: u32,
    ones: u32,
}

impl BitCount {
    fn add_one(self) -> BitCount {
        BitCount {
            zeros: self.zeros,
            ones: self.ones + 1,
        }
    }
    fn add_zero(self) -> BitCount {
        BitCount {
            zeros: self.zeros + 1,
            ones: self.ones,
        }
    }
    fn new() -> BitCount {
        BitCount { zeros: 0, ones: 0}
    }
}

fn index_diagnostic_data (mut state: Diagnosticts, next:DiagnosticDataPoint) ->  Diagnosticts {
    if state.size.is_none() {
        state.size = Some(next.size);
    }
    for power in 0..usize::BITS as usize{
        let cmp = usize::pow(2, power as u32);
        let count = &state.data[power];
        if next.value & cmp == cmp {
            state.data[power] = count.add_one()
        } else {
            state.data[power] = count.add_zero()
        }
    }
    return state;
}

// Returns tuple of size of input and the parsed number.
fn parse_point_or_panic(input: String) -> DiagnosticDataPoint {
    let size = input.len();
    match usize::from_str_radix(&input, 2) {
        Ok(n) => {
            return DiagnosticDataPoint {
                size: size,
                value: n
            };
        }
        Err(_) => {
            println!("Error parsing line {}", input);
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
