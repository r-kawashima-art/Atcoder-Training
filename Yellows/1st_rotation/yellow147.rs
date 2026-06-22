use proconio::input;
use std::collections::HashSet;
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}
impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self {
            x, y
        }
    }
    fn quadrant_half(&self) -> usize {
        /* Returns 0 if it is on the upper quadrants, 1 if it is on the lower parts */
        if self.y > 0 {
            return 0;
        } else if self.y == 0 {
            if self.x > 0 {
                return 0;
            } else {
                return 1;
            }
        } else {
            return 1;
        }
    }
    fn get_quadrant(&self) -> usize {
        /* Returns the quadrant of the point */
        if self.y > 0 {
            if self.x > 0 {
                return 1;
            } else {
                return 2;
            }
        } else {
            if self.x <= 0 {
                return 3;
            } else {
                return 4;
            }
        }
    }
    fn cross(&self, other: Point) -> i64 {
        /* caclaculates the norm of the cross vector by setting the pivot as (0, 0) */
        let cross_norm = self.x * other.y - self.y * other.x;
        return cross_norm;
    }
    fn dot(&self, other: Point) -> i64 {
        let inner_product: i64 = self.x * other.x + self.y * other.y;
        return inner_product;
    }
    fn square_distance(&self, other: Point) -> i64 {
        let squared: i64 = (other.x - self.x) * (other.x - self.x) + (other.y - self.y) * (other.y - self.y);
        return squared;
    }
    fn arg_sort(&self, points: &mut Vec<Point>) {
        use std::cmp::Ordering;
        points.sort_by(|point1, point2| {
            let half1: usize = point1.quadrant_half();
            let half2: usize = point2.quadrant_half();
            // Check if the quadrants are different.
            if half1 != half2 {
                return half1.cmp(&half2);
            }
            let cross_norm = point1.cross(*point2);
            if cross_norm != 0 {
                if cross_norm > 0 {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }
            let square_distance1: i64 = point1.square_distance(Point::new(0, 0));
            let square_distance2: i64 = point2.square_distance(Point::new(0, 0));
            return square_distance1.cmp(&square_distance2);
        });
    }

}
fn main() {
    input!{n: usize, xy: [(i64, i64); n]}
    let mut acute_triangles: usize = 0;
    let mut right_triangles: usize = 0;
    for center in 0..n {
        let mut vectors: Vec<Point> = Vec::new();
        let mut set: HashSet<(i64, i64)> = HashSet::new();
        for i in 0..n {
            if center != i {
                vectors.push(Point::new(xy[i].0 - xy[center].0, xy[i].1 - xy[center].1));
                set.insert((xy[i].0 - xy[center].0, xy[i].1 - xy[center].1));
            }
        }
        let pivot: Point = Point::new(0, 0);
        pivot.arg_sort(&mut vectors);
        let n_vectors: usize = vectors.len();
        for base in 0..n_vectors {
            let mut acute_end: usize = base + 1;
            let mut right_end: usize = base + 1;
            let mut obtuse_end: usize = base + 1;
            
            // Find boundary where dot product becomes <= 0 (acute region ends)
            while acute_end < n_vectors && vectors[base].dot(vectors[acute_end]) > 0 {
                acute_end += 1;
            }
            
            // Find boundary where dot product becomes < 0 (right angle region ends)
            right_end = acute_end;
            while right_end < n_vectors && vectors[base].dot(vectors[right_end]) == 0 {
                right_end += 1;
            }
            
            // Find boundary where dot product becomes strictly negative (obtuse region ends)
            obtuse_end = right_end;
            while obtuse_end < n_vectors && vectors[base].dot(vectors[obtuse_end]) >= 0 {
                obtuse_end += 1;
            }
            
            acute_triangles += acute_end - (base + 1);
            right_triangles += right_end - acute_end;
        }
    }
    let obtuse_triangles = n * (n - 1) * (n - 2) / 6 - right_triangles - acute_triangles;
    println!("{} {} {}", acute_triangles, right_triangles, obtuse_triangles);
}
