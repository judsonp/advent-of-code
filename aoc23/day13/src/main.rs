use std::fs;

struct Field {
    rows: Vec<u64>,
    cols: Vec<u64>,
}

fn field_elem_set(elem: &mut u64, pos: usize) {
    *elem |= 0x1 << pos;
}

fn main() {
    let input_s = fs::read_to_string("inputs/day13.txt").unwrap();
    let input = parse_input(&input_s);
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &[Field]) -> usize {
    input
        .iter()
        .map(|field| smudged_field_score(field, 0))
        .sum()
}

fn part_two(input: &[Field]) -> usize {
    input
        .iter()
        .map(|field| smudged_field_score(field, 1))
        .sum()
}

fn smudged_field_score(field: &Field, target: u32) -> usize {
    if let Some(h) = find_smudged_reflect(&field.cols, target) {
        return h;
    }
    if let Some(v) = find_smudged_reflect(&field.rows, target) {
        return 100 * v;
    }
    panic!("No reflection!");
}

fn find_smudged_reflect(elems: &Vec<u64>, target: u32) -> Option<usize> {
    (1..elems.len()).find(|idx| has_smudged_reflect(elems, *idx, target))
}

fn has_smudged_reflect(elems: &Vec<u64>, idx: usize, target: u32) -> bool {
    let left = (0..idx).rev();
    let right = idx..elems.len();
    let diffcount = left
        .zip(right)
        .map(|(i1, i2)| elems[i1] ^ elems[i2])
        .map(|x| x.count_ones())
        .sum::<u32>();
    diffcount == target
}

fn parse_input(input: &str) -> Vec<Field> {
    input.trim().split("\n\n").map(parse_field).collect()
}

fn parse_field(input: &str) -> Field {
    let row_count = input.trim().split('\n').count();
    let col_count = input.trim().split('\n').next().unwrap().len();
    let mut rows: Vec<u64> = vec![0; row_count];
    let mut cols: Vec<u64> = vec![0; col_count];

    for (row, line) in input.trim().split('\n').enumerate() {
        for (col, symbol) in line.chars().enumerate() {
            if symbol == '#' {
                field_elem_set(&mut rows[row], col);
                field_elem_set(&mut cols[col], row);
            }
        }
    }

    Field { rows, cols }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input = "#.#\n##.\n..#\n...";
        let parsed = parse_field(input);
        assert_eq!(parsed.rows, vec![5, 3, 4, 0]);
        assert_eq!(parsed.cols, vec![3, 2, 5]);
    }

    #[test]
    fn example() {
        let input_s = fs::read_to_string("../examples/day13.txt").unwrap();
        let input = parse_input(&input_s);
        assert_eq!(part_one(&input), 405);
    }

    #[test]
    fn example_part2() {
        let input_s = fs::read_to_string("../examples/day13.txt").unwrap();
        let input = parse_input(&input_s);
        assert_eq!(part_two(&input), 400);
    }
}
