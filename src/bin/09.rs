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
        .filter(|&rect| !any_edge_cuts_rectangle(&edges, rect))
        // .inspect(|&(a, b)| eprintln!("(a, b) = {a:#?} {b:#?},  {}", area(a, b)))
        .map(|(a, b)| area(a, b))
        .max();
    max
}

fn find_edges(points: &Vec<P>) -> Vec<Edge> {
    let mut edges = vec![];
    for ps in points
        .windows(2)
        .chain(std::iter::once(&[*points.last().unwrap(), points[0]][..]))
    {
        let &[start, end] = ps else {
            panic!("windows didn't return size 2")
        };
        // eprintln!("{:#?} {:#?}", p1, p2);
        edges.push(Edge { start, end });
    }
    edges
}

#[derive(Debug, PartialEq)]
struct Edge {
    start: P,
    end: P,
}

#[derive(Debug, PartialEq)]
struct VerticalEdge {
    x: usize,
    y_start: usize,
    y_end: usize,
}

impl VerticalEdge {
    fn new(x: usize, y_start: usize, y_end: usize) -> Self {
        Self { x, y_start, y_end }
    }
}

impl Edge {
    fn new(start: P, end: P) -> Self {
        Self { start, end }
    }
}

trait Rectangle {
    fn transpose(&self) -> (P, P);
}

impl Rectangle for (P, P) {
    fn transpose(&self) -> (P, P) {
        (P::new(self.0.y, self.0.x), P::new(self.1.y, self.1.x))
    }
}

#[derive(Clone, Copy, PartialEq)]
enum CornerFlag {
    None,
    Low,
    High,
}

/// Edges are sorted
fn any_edge_cuts_rectangle(edges: &[Edge], mut rectangle: (P, P)) -> bool {
    rectangle = normalise_corners(rectangle.0, rectangle.1);
    for edge in edges {
        // transpose into vertical edge

        let mut test_rectangle = rectangle;
        let vertical_edge = if edge.start.y == edge.end.y {
            // edge is horizontal
            test_rectangle = rectangle.transpose();
            VerticalEdge {
                x: edge.start.y,
                y_start: edge.end.x,
                y_end: edge.start.x,
            }
        } else {
            VerticalEdge {
                x: edge.start.x,
                y_start: edge.start.y,
                y_end: edge.end.y,
            }
        };

        if edge_cuts_rectangle(vertical_edge, test_rectangle) {
            return true;
        }
        // hold that the right side of the edge is inside
    }
    return false;
}

/// Valid
/// E
/// |    #
/// |
/// |#
/// |
/// S
///
fn edge_cuts_rectangle(edge: VerticalEdge, rectangle: (P, P)) -> bool {
    if edge.x < rectangle.0.x || edge.x > rectangle.1.x {
        return false;
    }

    // ax <= edge.x <= bx

    let y_min = edge.y_start.min(edge.y_end);
    let y_max = edge.y_start.max(edge.y_end);

    let ends_before = y_max <= rectangle.0.y;
    let starts_after = y_min >= rectangle.1.y;
    if (ends_before || starts_after) && !(ends_before && starts_after) {
        return false;
    }

    // edge overlaps rectangle
    // it cuts the rectangle if it is NOT the left edge (when running start->end upwards)
    // so right edge if running downwards

    let left_edge = if edge.y_start > edge.y_end {
          rectangle.0.x
    } else {
          rectangle.1.x
    };
    return edge.x != left_edge;
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
    fn test_find_edges() {
        let points = parse(&advent_of_code::template::read_file("examples", DAY));

        let edges = find_edges(&points);

        assert_eq!(
            edges,
            vec![
                Edge::new(P::new(7, 1), P::new(11, 1)),
                Edge::new(P::new(11, 1), P::new(11, 7)),
                Edge::new(P::new(11, 7), P::new(9, 7)),
                Edge::new(P::new(9, 7), P::new(9, 5)),
                Edge::new(P::new(9, 5), P::new(2, 5)),
                Edge::new(P::new(2, 5), P::new(2, 3)),
                Edge::new(P::new(2, 3), P::new(7, 3)),
                Edge::new(P::new(7, 3), P::new(7, 1)),
            ]
        )
    }

    #[test]
    fn test_single_cut() {
        assert!(edge_cuts_rectangle(
            VerticalEdge::new(7, 3, 1),
            (P::new(2, 3), P::new(7, 1))
        ))
    }

    #[test]
    fn test_cuts() {
        let points = parse(&advent_of_code::template::read_file("examples", DAY));

        let edges = find_edges(&points);

        assert!(any_edge_cuts_rectangle(&edges, (P::new(2, 3), P::new(7, 1))) == true);
        assert!(any_edge_cuts_rectangle(&edges, (P::new(2, 5), P::new(7, 3))) == false);
        assert!(any_edge_cuts_rectangle(&edges, (P::new(6, 3), P::new(8, 10))) == true);
        assert!(any_edge_cuts_rectangle(&edges, (P::new(9, 7), P::new(11, 1))) == false);
        assert!(any_edge_cuts_rectangle(&edges, (P::new(9, 7), P::new(7, 3))) == true);
    }
}
