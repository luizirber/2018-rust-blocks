struct Example {
    num: u32,
    mins: Vec<u64>,
}

impl Example {

    pub fn merge(&mut self, other: &Example) {
        let max_size = self.mins.len() + other.mins.len();
        let mut merged: Vec<u64> = Vec::with_capacity(max_size);

        let mut self_iter = self.mins.iter();
        let mut other_iter = other.mins.iter();

        let mut self_value = self_iter.next();
        let mut other_value = other_iter.next();
        while !self_value.is_none() {
            let value = self_value.unwrap();
            match other_value {
                None => {
                    merged.push(*value);
                    merged.extend(self_iter);
                    break;
                }
                Some(x) if x < value => {
                    merged.push(*x);
                    other_value = other_iter.next();
                }
                Some(x) if x == value => {
                    merged.push(*x);
                    other_value = other_iter.next();
                    self_value = self_iter.next();
                }
                Some(x) if x > value => {
                    merged.push(*value);
                    self_value = self_iter.next();
                }
                Some(_) => {}
            }
        }
        if let Some(value) = other_value {
            merged.push(*value);
        }
        merged.extend(other_iter);

        if merged.len() < (self.num as usize) || (self.num as usize) == 0 {
            self.mins = merged;
        } else {
            self.mins = merged.iter()
                           .map(|&x| x as u64)
                           .take(self.num as usize)
                           .collect();
        }
    }

}

fn main() {
    let mut vec1: Vec<u64> = Vec::with_capacity(4);
    let mut vec2: Vec<u64> = Vec::with_capacity(4);

    for i in 1..9 {
        if i % 2 == 0 {
            vec1.push(i);
        } else {
            vec2.push(i);
        }
    }

    let mut st1 = Example { num: 4, mins: vec1 };
    let st2 = Example { num: 4, mins: vec2 };

    println!("{:?}", st1.mins);
    println!("{:?}", st2.mins);

    st1.merge(&st2);

    println!("{:?}", st1.mins);
}
