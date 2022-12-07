use std::collections::{HashSet, VecDeque};

const DATA: &str = include_str!("../../data/day6.txt");

#[derive(Debug, Default)]
struct Scanner<const N: usize> {
    buffer: VecDeque<char>,
    received: usize,
}

impl<const N: usize> Scanner<N> {
    pub fn received(&mut self, c: char) {
        if self.buffer.len() >= N {
            self.buffer.pop_front();
        }
        self.buffer.push_back(c);
        self.received += 1;
    }

    pub fn unique_count(&self) -> usize {
        let set: HashSet<char> = self.buffer.iter().copied().collect();
        set.len()
    }

    pub fn received_count(&self) -> usize {
        self.received
    }

    pub fn run_scanner(data: &str) -> Option<usize> {
        let mut scanner = Scanner::<N>::default();
        for c in data.chars() {
            scanner.received(c);
            if scanner.unique_count() == N {
                return Some(scanner.received_count());
            }
        }
        None
    }
}

fn main() {
    let received_count = Scanner::<4>::run_scanner(DATA);
    println!("characters processed = {:?}", received_count);

    let received_count = Scanner::<14>::run_scanner(DATA);
    println!("characters processed = {:?}", received_count);
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_scanner_for_data<const N: usize>(expected: usize, data: &str) {
        let received_count = Scanner::<N>::run_scanner(data);
        assert_eq!(received_count, Some(expected));
    }

    #[test]
    fn test_scanner() {
        test_scanner_for_data::<4>(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        test_scanner_for_data::<4>(5, "bvwbjplbgvbhsrlpgdmjqwftvncz");
        test_scanner_for_data::<4>(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        test_scanner_for_data::<4>(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");

        test_scanner_for_data::<14>(19, "mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        test_scanner_for_data::<14>(23, "bvwbjplbgvbhsrlpgdmjqwftvncz");
        test_scanner_for_data::<14>(23, "nppdvjthqldpwncqszvftbrmjlhg");
        test_scanner_for_data::<14>(29, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        test_scanner_for_data::<14>(26, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
    }
}
