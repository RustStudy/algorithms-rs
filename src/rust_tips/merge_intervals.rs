// 自定义复合数据合并Range

use std::cmp;
use std::ops::Range;

struct MergedRanges<I> {
    values: I,
    last: Option<Range<i32>>
}

fn merge_ranges<I>(iterator: I) -> MergedRanges<I::IntoIter>
    where I: IntoIterator<Item=Range<i32>>
{
    let mut iterator = iterator.into_iter();
    let last = iterator.next();

    MergedRanges {
        values: iterator,
        last: last,
    }
}

impl<I> Iterator for MergedRanges<I>
    where I: Iterator<Item=Range<i32>>
{
    type Item = Range<i32>;

    fn next(&mut self) -> Option<Range<i32>> {
        if let Some(mut last) = self.last.clone() {
            for new in &mut self.values {
                if last.end < new.start {
                    self.last = Some(new);
                    return Some(last);
                }

                last.end = cmp::max(last.end, new.end);
            }

            self.last = None;
            return Some(last);
        }
        None
    }
}

fn main() {
    let mut v = vec![3..6, 1..5, 7..11, 9..12, 4..8];

    v.sort_by(|left, right| left.start.cmp(&right.start));
    let merged: Vec<_> = merge_ranges(v).collect();

    for range in &merged {
        print!(" {:?}", range);
    }
    println!("");
}
