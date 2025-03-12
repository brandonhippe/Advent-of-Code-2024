use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct FileBlock {
    start: i64,
    size: i64,
    id: i64,
}

impl FileBlock {
    fn new(start: i64, size: i64, id: i64) -> FileBlock {
        FileBlock {
            start: start,
            size: size,
            id: id,
        }
    }

    fn checksum(&self) -> i64 {
        (self.start..self.start+self.size).map(|n| n * self.id).sum::<i64>()
    }
}

fn part1(contents: String) -> i64 {
    let mut blocks: Vec<FileBlock> = Vec::new();
    let mut pos: i64 = 0;
    for (ix, n) in contents.lines().next().unwrap().chars().enumerate() {
        let size = (n as u32 - '0' as u32) as i64;
        if ix % 2 == 0 {
            blocks.push(FileBlock::new(pos, size, ix as i64 / 2));
        }
        pos += size;
    }

    let mut start_ix: usize = 0;
    let mut end_ix: usize = blocks.len() - 1;
    let mut free: i64 = 0;

    loop {
        while start_ix < end_ix {
            free = blocks[start_ix + 1].start - (blocks[start_ix].start + blocks[start_ix].size);
            if free > 0 {
                break;
            }
            start_ix += 1;
        }

        if start_ix >= end_ix {
            break;
        }
        
        let moving = free.min(blocks[end_ix].size);
        blocks.insert(start_ix + 1, FileBlock::new(blocks[start_ix].start + blocks[start_ix].size, moving, blocks[end_ix].id));
        start_ix += 1;
        end_ix += 1;

        blocks[end_ix].size -= moving;
        if blocks[end_ix].size == 0 {
            blocks.pop();
            end_ix -= 1;
        }
    }

    return blocks.iter().map(|b| b.checksum()).sum::<i64>();
}

fn part2(contents: String) -> i64 {
    let mut blocks: Vec<FileBlock> = Vec::new();
    let mut pos: i64 = 0;
    for (ix, n) in contents.lines().next().unwrap().chars().enumerate() {
        let size = (n as u32 - '0' as u32) as i64;
        if ix % 2 == 0 {
            blocks.push(FileBlock::new(pos, size, ix as i64 / 2));
        }
        pos += size;
    }

    for block_id in blocks.clone().iter().map(|b| b.id).rev() {
        let (moving_ix, _) = blocks.iter().enumerate().filter(|(_, b)| b.id == block_id).next().unwrap();
        let mut moving_block = blocks[moving_ix];
        if let Some(insert_ix) = blocks.iter().take(moving_ix).enumerate().filter_map(|(ix, b)| {
            if moving_block.size <= blocks[ix + 1].start - (b.start + b.size) {
                Some(ix)
            } else {
                None
            }
        }).next() {
            moving_block.start = blocks[insert_ix].start + blocks[insert_ix].size;
            blocks.remove(moving_ix);
            blocks.insert(insert_ix + 1, moving_block);
        }
    }

    return blocks.iter().map(|b| b.checksum()).sum::<i64>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 1928);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 2858);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "9".to_string();

    let root = env::current_dir().unwrap();
    let path_str = if args.len() > 1 {
        args[1].clone()
    } else if root.ends_with(format!("{}", day)) {
        format!("../../../Inputs/{}_{}.txt", year, day)
    } else {
        format!("/Inputs/{}_{}.txt", year, day)
    };

    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nChecksum: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nChecksum: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}