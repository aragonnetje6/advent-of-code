use std::iter::repeat_n;

use itertools::Itertools;
use nom::{
    character::complete,
    combinator::{map, map_opt, opt},
    multi::many1,
    sequence::pair,
    IResult,
};

fn single_digit(input: &str) -> IResult<&str, usize> {
    map_opt(complete::one_of("0123456789"), |x| {
        x.to_digit(10).map(|x| x as usize)
    })(input)
}

fn parse_pair(input: &str) -> IResult<&str, (usize, usize)> {
    pair(single_digit, single_digit)(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<usize>, Vec<usize>)> {
    map(
        pair(many1(parse_pair), opt(single_digit)),
        |(pairs, maybe_file)| {
            let (mut files, gaps): (Vec<usize>, Vec<usize>) = pairs.into_iter().unzip();
            if let Some(file) = maybe_file {
                files.push(file);
            }
            (files, gaps)
        },
    )(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileChunk {
    id: usize,
}

fn expand(files: &[usize], gaps: &[usize]) -> Vec<Option<FileChunk>> {
    files
        .iter()
        .zip(gaps.iter())
        .enumerate()
        .flat_map(|(id, (fsize, gsize))| {
            repeat_n(Some(FileChunk { id }), *fsize).chain(repeat_n(None, *gsize))
        })
        .chain(if files.len() > gaps.len() {
            repeat_n(
                Some(FileChunk {
                    id: files.len() - 1,
                }),
                *files.last().expect("no files"),
            )
        } else {
            repeat_n(None, 0)
        })
        .collect()
}

fn compress_chunks(chunks: &mut Vec<Option<FileChunk>>) {
    while chunks.iter().any(Option::is_none) {
        let item = chunks.pop().expect("empty");
        if let Some(ptr) = chunks.iter_mut().find(|x| x.is_none()) {
            *ptr = item;
        }
    }
}

fn checksum(chunks: &[Option<FileChunk>]) -> usize {
    chunks
        .iter()
        .enumerate()
        .map(|(i, chunk)| chunk.map(|x| x.id * i).unwrap_or_default())
        .sum()
}

pub fn part1(input: &str) -> String {
    let (_, (files, gaps)) = parse_input(input).expect("parsing error");
    let mut chunks = expand(&files, &gaps);
    compress_chunks(&mut chunks);
    checksum(&chunks).to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum File {
    Frozen { size: usize, id: usize },
    Unfrozen { size: usize, id: usize },
}

impl File {
    const fn size(self) -> usize {
        match self {
            Self::Frozen { size, .. } | Self::Unfrozen { size, .. } => size,
        }
    }

    const fn id(self) -> usize {
        match self {
            Self::Frozen { size: _, id } | Self::Unfrozen { size: _, id } => id,
        }
    }

    const fn frozen(self) -> Self {
        match self {
            Self::Frozen { size, id } | Self::Unfrozen { size, id } => Self::Frozen { size, id },
        }
    }
}

fn compress_files(files: &mut Vec<File>, gaps: &mut Vec<usize>) {
    while let Some((file_index, &file)) = files
        .iter()
        .enumerate()
        .rev()
        .find(|(_, x)| matches!(x, File::Unfrozen { .. }))
    {
        let size = file.size();
        if let Some((gap_index, _)) = gaps.iter().find_position(|x| **x >= size) {
            if gap_index < file_index {
                files.remove(file_index);
                files.insert(gap_index + 1, file.frozen());
                gaps[file_index - 1] = gaps[file_index - 1]
                    + gaps.get(file_index).copied().unwrap_or_default()
                    + file.size();
                if gaps.len() > file_index {
                    gaps.remove(file_index);
                }
                gaps[gap_index] -= file.size();
                gaps.insert(gap_index, 0);
            } else {
                files[file_index] = file.frozen();
            }
        } else {
            files[file_index] = file.frozen();
        }
    }
}

fn checksum_files(files: &[File], gaps: &[usize]) -> usize {
    files
        .iter()
        .zip(gaps.iter())
        .flat_map(|(file, gap)| {
            repeat_n(Some(FileChunk { id: file.id() }), file.size()).chain(repeat_n(None, *gap))
        })
        .chain(if files.len() > gaps.len() {
            let last = files.last().expect("impossible");
            repeat_n(Some(FileChunk { id: last.id() }), last.size())
        } else {
            repeat_n(None, 0)
        })
        .enumerate()
        .map(|(i, chunk)| chunk.map(|x| x.id * i).unwrap_or_default())
        .sum()
}

pub fn part2(input: &str) -> String {
    let (_, (files, mut gaps)) = parse_input(input).expect("parsing error");
    let mut wrapped_files = files
        .into_iter()
        .enumerate()
        .map(|(id, size)| File::Unfrozen { id, size })
        .collect();
    compress_files(&mut wrapped_files, &mut gaps);
    checksum_files(&wrapped_files, &gaps).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 1928.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 2858.to_string());
    }
}
