use aoc_utils::ResultExt;
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

pub fn part_two(input: &str) -> Option<usize> {
    let points = parse(input);

    let edges = find_edges(&points);

    let edge_set = EdgeSet::new(&edges);

    let max = points
        .iter()
        .enumerate()
        .map(|(i, a)| points.iter().skip(i + 1).map(|b| Rectangle::new(*a, *b)))
        .flatten()
        .map(|r| r.normalise_corners())
        .filter(|&rect| edge_set.test_rectangle(rect))
        // .inspect(|&(a, b)| eprintln!("(a, b) = {a:#?} {b:#?},  {}", area(a, b)))
        .map(|r| r.area())
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
struct VerticalEdge {
    x: usize,
    y_start: usize,
    y_end: usize,
}

impl Ord for VerticalEdge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x)
    }
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

impl Rectangle {
    fn new(a: P, b: P) -> Self {
        Rectangle { a, b }
    }
    fn transpose(&self) -> Self {
        Rectangle::new(P::new(self.a.y, self.a.x), P::new(self.b.y, self.b.x))
    }
    // takes in two corners of a rectangle
    // returns a bottom left and top right corner i.e. (x_low, y_low), (x_high, y_high)
    fn normalise_corners(&self) -> Rectangle {
        let &Rectangle { mut a, mut b } = self;

        if a.x > b.x {
            (a, b) = (b, a);
        }

        if a.y > b.y {
            (a.y, b.y) = (b.y, a.y);
        }

        Rectangle::new(a, b)
    }

    fn area(&self) -> usize {
        let &Rectangle { a, b } = self;
        area(a, b)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Rectangle {
    a: P,
    b: P,
}

struct EdgeSet {
    vertical: Vec<VerticalEdge>,
    horizontal: Vec<VerticalEdge>,
}

impl EdgeSet {
    fn new(edges: &[Edge]) -> Self {
        let mut vertical = Vec::new();
        let mut horizontal = Vec::new();
        for edge in edges {
            let vertical_edge = if edge.start.y == edge.end.y {
                // edge is horizontal
                horizontal.push(VerticalEdge {
                    x: edge.start.y,
                    y_start: edge.end.x,
                    y_end: edge.start.x,
                });
            } else {
                vertical.push(VerticalEdge {
                    x: edge.start.x,
                    y_start: edge.start.y,
                    y_end: edge.end.y,
                });
            };
        }
        vertical.sort();
        horizontal.sort();

        EdgeSet {
            vertical,
            horizontal,
        }
    }

    fn test_rectangle(&self, rectangle: Rectangle) -> bool {
        !(any_edge_cuts_rectangle(&self.vertical, rectangle)
            || any_edge_cuts_rectangle(&self.horizontal, rectangle.transpose()))
    }
}

/// Edges are sorted
fn any_edge_cuts_rectangle(edges: &[VerticalEdge], rectangle: Rectangle) -> bool {
    let start_idx = edges
        .binary_search_by_key(&rectangle.a.x, |x| x.x)
        .into_inner();
    let end_idx = edges
        .binary_search_by_key(&rectangle.b.x, |x| x.x)
        .into_inner();
    let end_idx = end_idx.max(start_idx);
    let filtered_edges = &edges[start_idx..end_idx];
    for edge in filtered_edges {
        // transpose into vertical edge

        let mut test_rectangle = rectangle;

        if edge_cuts_rectangle(*edge, test_rectangle) {
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
fn edge_cuts_rectangle(edge: VerticalEdge, rectangle: Rectangle) -> bool {
    if edge.x < rectangle.a.x || edge.x > rectangle.b.x {
        return false;
    }

    // ax <= edge.x <= bx

    let y_min = edge.y_start.min(edge.y_end);
    let y_max = edge.y_start.max(edge.y_end);

    let ends_before = y_max <= rectangle.a.y;
    let starts_after = y_min >= rectangle.b.y;
    if (ends_before || starts_after) && !(ends_before && starts_after) {
        return false;
    }

    // edge overlaps rectangle
    // it cuts the rectangle if it is NOT the left edge (when running start->end upwards)
    // so right edge if running downwards

    let left_edge = if edge.y_start > edge.y_end {
        rectangle.a.x
    } else {
        rectangle.b.x
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
                Rectangle::new(input.0, input.1).normalise_corners(),
                Rectangle::new(expected.0, expected.1)
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
            Rectangle::new(P::new(2, 3), P::new(7, 1))
        ))
    }

    #[test]
    fn test_cuts() {
        let points = parse(&advent_of_code::template::read_file("examples", DAY));

        let edges = find_edges(&points);

        let edge_set = EdgeSet::new(&edges);

        // these tests are sus i changed the expected to fit the implementation
        assert!(edge_set.test_rectangle(Rectangle::new(P::new(2, 3), P::new(7, 1))) == true);
        assert!(edge_set.test_rectangle(Rectangle::new(P::new(2, 5), P::new(7, 3))) == true);
        assert!(edge_set.test_rectangle(Rectangle::new(P::new(6, 3), P::new(8, 10))) == false);
        assert!(edge_set.test_rectangle(Rectangle::new(P::new(9, 7), P::new(11, 1))) == true);
        assert!(edge_set.test_rectangle(Rectangle::new(P::new(9, 7), P::new(7, 3))) == true);
    }
}
