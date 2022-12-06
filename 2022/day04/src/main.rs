use std::cmp::{max, min};
use std::ops::RangeInclusive;

trait RangeInclusiveContainsRange {
    fn contains_range(&self, other: &Self) -> bool;
}

impl<T> RangeInclusiveContainsRange for RangeInclusive<T>
where
    T: Ord,
{
    fn contains_range(&self, other: &Self) -> bool {
        (self.start() <= other.start()) && (other.end() <= self.end())
    }
}

trait RangeInclusiveIntersect: Sized {
    fn intersect(&self, other: &Self) -> Option<Self>;
}

impl<T> RangeInclusiveIntersect for RangeInclusive<T>
where
    T: Copy + Ord,
{
    fn intersect(&self, other: &Self) -> Option<Self> {
        let start = max(*self.start(), *other.start());
        let end = min(*self.end(), *other.end());
        if start > end {
            None
        } else {
            Some(start..=end)
        }
    }
}

fn main() {
    let input_lines = util::get_input_lines();

    let contained_count = input_lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(',').unwrap())
        .map(|(left, right)| {
            let (left_start, left_end) = left.split_once('-').unwrap();
            let (right_start, right_end) = right.split_once('-').unwrap();
            (
                left_start.parse::<i32>().unwrap()..=left_end.parse::<i32>().unwrap(),
                right_start.parse::<i32>().unwrap()..=right_end.parse::<i32>().unwrap(),
            )
        })
        .filter_map(|(left_range, right_range)| {
            if left_range.contains_range(&right_range) || right_range.contains_range(&left_range) {
                Some(())
            } else {
                None
            }
        })
        .count();
    println!("Contained ranges: {}", contained_count);

    let overlapped_count = input_lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(',').unwrap())
        .map(|(left, right)| {
            let (left_start, left_end) = left.split_once('-').unwrap();
            let (right_start, right_end) = right.split_once('-').unwrap();
            (
                left_start.parse::<i32>().unwrap()..=left_end.parse::<i32>().unwrap(),
                right_start.parse::<i32>().unwrap()..=right_end.parse::<i32>().unwrap(),
            )
        })
        .filter_map(|(left_range, right_range)| left_range.intersect(&right_range))
        .count();
    println!("Overlapped ranges: {}", overlapped_count);
}
