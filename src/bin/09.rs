/// TODO:
/// to check if a rectangle is valid traverse points in a positive orientation; check if any edge cuts through the middle of the rectangle.
///
/// I think the orientation i.e. knowing that the right side of the edge is always inside is needed as the edge may cut through the rectangle if it is on the edge of the rectangle.
use lina::{Point2, Vec2};

advent_of_code::solution!(9);

type P = Point2<usize>;

fn parse(input: &str) -> Vec<P> {
    input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(",").expect("Each line has num,num");
            Point2::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let points = parse(input);

    points
        .iter()
        .enumerate()
        .map(|(i, a)| points.iter().skip(i + 1).map(|b| (*a, *b)))
        .flatten()
        .map(|(a, b)| (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        .max()
}

// takes in two corners of a rectangle
// returns a bottom left and top right corner i.e. (x_low, y_low), (x_high, y_high)
fn normalise_corners(mut a: P, mut b: P) -> (P, P) {
    if a.x > b.x {
        (a, b) = (b, a);
    }

    if a.y > b.y {
        (a.y, b.y) = (b.y, a.y);
    }

    (a, b)
}

pub fn part_two(input: &str) -> Option<usize> {
    let points = parse(input);

    let edges = find_edges(&points);

    let max = points
        .iter()
        .enumerate()
        .map(|(i, a)| points.iter().skip(i + 1).map(|b| (*a, *b)))
        .flatten()
        .map(|(a, b)| normalise_corners(a, b))
        .filter(|&rect| !any_other_point_inside(rect, &points))
        .filter(|&(a, b)| {
            let inside_point = a + Vec2::new(1, 1);
            // eprintln!(
            //     "{:#?}, {:#?} := {inside_point:#?}: {}",
            //     a,
            //     b,
            //     is_point_inside(&edges, inside_point)
            // );
            is_point_inside(&edges, inside_point)
        })
        .inspect(|&(a, b)| eprintln!("(a, b) = {a:#?} {b:#?},  {}", area(a, b)))
        .map(|(a, b)| area(a, b))
        .max();
    max
}

fn any_other_point_inside(rectangle: (P, P), points: &[P]) -> bool {
    let (a, b) = rectangle;
    eprintln!();
    points
        .iter()
        // .filter(|&&p| p != a && p != b)
        // filter edges
        // .filter(|&&p| !((p.x == a.x || p.x == b.x) || (p.y == a.y || p.y == b.y)))
        .inspect(|&&p| eprintln!("{p:#?} {}", (rectangle_contains(a, b, p))))
        .any(|&p| rectangle_contains(a, b, p))
}

fn find_edges(points: &Vec<P>) -> Vec<VerticalEdge> {
    let mut edges = vec![];
    for ps in points
        .windows(2)
        .chain(std::iter::once(&[points[0], *points.last().unwrap()][..]))
    {
        let [p1, p2] = ps else {
            panic!("windows didn't return size 2")
        };
        // eprintln!("{:#?} {:#?}", p1, p2);
        if p1.x == p2.x {
            edges.push(VerticalEdge {
                x: p1.x,
                y_low: p1.y.min(p2.y),
                y_high: p1.y.max(p2.y),
            });
        }
    }
    edges.sort_unstable_by_key(|x| x.x);
    edges
}

#[derive(Debug, PartialEq)]
struct VerticalEdge {
    x: usize,
    y_low: usize,
    y_high: usize,
}

impl VerticalEdge {
    fn new(x: usize, y_low: usize, y_high: usize) -> Self {
        Self { x, y_low, y_high }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum CornerFlag {
    None,
    Low,
    High,
}

/// Edges are sorted
fn is_point_inside(edges: &[VerticalEdge], point: P) -> bool {
    use CornerFlag::*;
    let mut inside = false;
    let mut corner_flag: CornerFlag = None;
    for edge in edges {
        if edge.x > point.x {
            return inside;
        }
        let current_corner = {
            if edge.y_low == point.y {
                Low
            } else if edge.y_high == point.y {
                High
            } else {
                None
            }
        };
        if edge.y_low <= point.y && point.y <= edge.y_high {
            match (corner_flag, current_corner) {
                (None, _) | (_, None) | (High, High) | (Low, Low) => {
                    inside = !inside;
                }
                _ => {}
            }

            if current_corner != None && corner_flag != None {
                corner_flag = None;
            } else {
                corner_flag = current_corner;
            }
        }
    }
    return inside;
}

fn rectangle_contains(mut a: P, mut b: P, p: P) -> bool {
    (a.x < p.x && a.y < p.y && p.x < b.x && p.y < b.y)
}

fn area(a: P, b: P) -> usize {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_normalise_rec() {
        let ins = [
            (
                (Point2::new(1, 1), Point2::new(5, 5)),
                (Point2::new(1, 1), Point2::new(5, 5)),
            ),
            (
                (Point2::new(1, 5), Point2::new(5, 1)),
                (Point2::new(1, 1), Point2::new(5, 5)),
            ),
            (
                (Point2::new(5, 5), Point2::new(1, 1)),
                (Point2::new(1, 1), Point2::new(5, 5)),
            ),
        ];
        for (input, expected) in ins {
            assert_eq!(
                normalise_corners(input.0, input.1),
                (expected.0, expected.1)
            );
        }
    }

    #[test]
    fn test_valid() {
        let points = parse(&advent_of_code::template::read_file("examples", DAY));

        let test_rec = (P::new(2, 3), P::new(9, 5));

        assert!(!any_other_point_inside(test_rec, &points));
        assert!(any_other_point_inside(
            (P::new(2, 1), P::new(11, 5)),
            &points
        ));
        assert!(any_other_point_inside(
            (P::new(2, 3), P::new(9, 7)),
            &points
        ));
    }

    #[test]
    fn test_find_edges() {
        let points = parse(&advent_of_code::template::read_file("examples", DAY));

        let edges = find_edges(&points);

        assert_eq!(
            edges,
            vec![
                VerticalEdge::new(2, 3, 5),
                VerticalEdge::new(7, 1, 3),
                VerticalEdge::new(9, 5, 7),
                VerticalEdge::new(11, 1, 7)
            ]
        )
    }

    #[test]
    fn test_inside() {
        let points = parse(&advent_of_code::template::read_file("examples", DAY));

        let edges = find_edges(&points);

        assert!(is_point_inside(&edges, P::new(1, 1)) == false);
        assert!(is_point_inside(&edges, P::new(5, 5)) == true);
        assert!(is_point_inside(&edges, P::new(10, 3)) == true);
        assert!(is_point_inside(&edges, P::new(12, 3)) == false);
        assert!(is_point_inside(&edges, P::new(12, 5)) == false);
        assert!(is_point_inside(&edges, P::new(10, 7)) == true);
    }
}
