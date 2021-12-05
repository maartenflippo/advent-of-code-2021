#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn is_at_45_deg(&self, other: Point) -> bool {
        let (dx, dy) = self.delta(other);
        
        dx == dy
    }

    pub fn delta(&self, other: Point) -> (u32, u32) {
        let dx = u32::abs_diff(self.x, other.x);
        let dy = u32::abs_diff(self.y, other.y);

        (dx, dy)
    }
}

impl From<&str> for Point {
    fn from(input: &str) -> Self {
        let components = input.split(',')
            .map(|num| num.parse::<u32>().expect("Failed to parse number."))
            .collect::<Vec<_>>();

        assert!(components.len() == 2, "A point is defined by 2 components.");

        Point {
            x: components[0],
            y: components[1],
        }
    }
}

impl From<(u32, u32)> for Point {
    fn from((x, y): (u32, u32)) -> Self {
        Point { x, y }
    }
}

#[derive(Hash)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn covers(&self, point: Point) -> bool {
        let (start_dx, start_dy) = self.start.delta(point);
        let (end_dx, end_dy) = self.end.delta(point);
        let (own_dx, own_dy) = self.start.delta(self.end);

        if own_dx == 0 && start_dx == 0 {
            let min_y = u32::min(self.start.y, self.end.y);
            let max_y = u32::max(self.start.y, self.end.y);

            min_y <= point.y && point.y <= max_y
        } else if own_dy == 0 && start_dy == 0 {
            let min_x = u32::min(self.start.x, self.end.x);
            let max_x = u32::max(self.start.x, self.end.x);

            min_x <= point.x && point.x <= max_x
        } else if self.is_diagonal() && self.start.is_at_45_deg(point) {
            start_dx + end_dx == own_dx && start_dy + end_dy == own_dy
        } else {
            false
        }
    }

    fn is_diagonal(&self) -> bool {
        self.start.is_at_45_deg(self.end)
    }
}

impl From<&str> for Line {
    fn from(input: &str) -> Self {
        let points = input.split(" -> ")
            .map(|p| p.into())
            .collect::<Vec<Point>>();

        assert!(points.len() == 2, "A line is defined by 2 points.");

        Self {
            start: points[0],
            end: points[1],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_construction() {
        let point = Point::from("1,2");
        assert_eq!(1, point.x);
        assert_eq!(2, point.y);
    }

    #[test]
    fn test_line_construction() {
        let line = Line::from("8,0 -> 0,8");

        assert_eq!(Point { x: 8, y: 0 }, line.start);
        assert_eq!(Point { x: 0, y: 8 }, line.end);
    }

    #[test]
    fn test_line_covers_correct_points() {
        let line = Line::from("0,9 -> 5,9");

        assert!(line.covers("0,9".into()));
        assert!(line.covers("1,9".into()));
        assert!(line.covers("2,9".into()));
        assert!(line.covers("3,9".into()));
        assert!(line.covers("4,9".into()));
        assert!(line.covers("5,9".into()));

        assert!(!line.covers("5,7".into()));
        assert!(!line.covers("5,8".into()));
        assert!(!line.covers("6,9".into()));
    }

    #[test]
    fn test_line_covers_correct_points_1() {
        let l1: Line = "7,0 -> 7,4".into();
        let l2: Line = "9,4 -> 3,4".into();

        let p = (7,4).into();
        assert!(l1.covers(p));
        assert!(l2.covers(p));
    }
}
