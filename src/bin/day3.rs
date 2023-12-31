use itertools::Itertools;

#[macro_use]
mod help;
fn main() {
    let data = input_to_str_vec!("3");
    let width = data[0].len();
    let height = data.len();
    let items = data
        .iter()
        .enumerate()
        .flat_map(|(y, d)| {
            d.chars()
                .enumerate()
                .group_by(|f| f.1 != '.')
                .into_iter()
                .filter(|(b, _)| *b)
                .flat_map(|(_, d)| {
                    d.group_by(|(_, c)| c.is_ascii_digit())
                        .into_iter()
                        .map(|(_, g)| g.collect_vec())
                        .map(|group| {
                            (
                                (group[0].0, y),
                                String::from_iter(group.iter().map(|g| g.1)),
                            )
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    let numbers = items
        .iter()
        .filter(|(_, f)| f.chars().all(|c| c.is_ascii_digit()))
        .collect_vec();

    let true_numbers = numbers
        .iter()
        .filter(|((x, y), s)| {
            let len = s.len();
            let mut checks = vec![];
            if *x > 0 {
                checks.push((x - 1, *y));
                if *y > 0 {
                    checks.push((x - 1, y - 1));
                }
                if y + 1 < height {
                    checks.push((x - 1, y + 1));
                }
            }
            if x + len < width {
                checks.push((x + len, *y));
                if *y > 0 {
                    checks.push((x + len, y - 1));
                }
                if y + 1 < height {
                    checks.push((x + len, y + 1));
                }
            }
            for i in *x..(x + len) {
                if *y > 0 {
                    checks.push((i, y - 1));
                }
                if y + 1 < height {
                    checks.push((i, y + 1));
                }
            }
            checks.into_iter().any(|(x, y)| {
                data[y].chars().nth(x).unwrap() != '.'
                    && !data[y].chars().nth(x).unwrap().is_ascii_digit()
            })
        })
        .collect_vec();

    part1!(true_numbers
        .iter()
        .map(|(_, s)| s.parse::<usize>().unwrap())
        .sum::<usize>());

    let stars = items
        .iter()
        .filter(|(_, f)| f.chars().all(|c| c == '*'))
        .collect_vec();

    assert!(stars.iter().all(|(_, f)| f.len() == 1));

    part2!(stars
        .iter()
        .map(|((x, y), _)| {
            let xo = ((*x as isize).saturating_sub(1) as usize)..=x + 1;
            let yo = ((*y as isize).saturating_sub(1) as usize)..=y + 1;
            let numbers = true_numbers
                .iter()
                .filter(|((nx, ny), ns)| {
                    let mut range = *nx..(nx + ns.len());
                    yo.contains(ny) && range.any(|x| xo.contains(&x))
                })
                .collect_vec();
            if numbers.len() == 2 {
                numbers
                    .into_iter()
                    .map(|(_, s)| s.parse::<usize>().unwrap())
                    .product()
            } else {
                0
            }
        })
        .sum::<usize>())
}
