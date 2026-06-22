use proconio::input;
// Q. What are the numbers of the acute, right and obtuse triangles?
// A. Angular sort + Two pointers algorithm
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
    fn angular_sort(&self, points: &mut Vec<Point>) {
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

fn arg_sort(vectors: &mut Vec<Point>) {
    vectors.sort_by(|a, b| {
        let v1 = if a.y > 0 || (a.y == 0 && a.x > 0) { 0 } else { 1 };
        let v2 = if b.y > 0 || (b.y == 0 && b.x > 0) { 0 } else { 1 };
        if v1 != v2 {
            v1.cmp(&v2)
        } else {
            // Use cross product to determine CCW order within the same half-plane
            0.cmp(&(a.x * b.y - a.y * b.x))
        }
    });
}

fn main() {
    input!{n: usize, xy: [(i64, i64); n]}
    
    let mut right_triangles: usize = 0;
    let mut obtuse_triangles: usize = 0;

    for center in 0..n {
        let mut vectors: Vec<Point> = Vec::new();
        for i in 0..n {
            if center != i {
                vectors.push(Point::new(xy[i].0 - xy[center].0, xy[i].1 - xy[center].1));
            }
        }
        
        arg_sort(&mut vectors);
        let n_vectors: usize = vectors.len();
        
        // Duplicate vectors to handle circular two-pointer easily
        let mut doubled = vectors.clone();
        doubled.extend(vectors);

        let mut p = 0;
        let mut q = 0;

        for base in 0..n_vectors {
            if p <= base { p = base + 1; }
            if q <= p { q = p; }

            // Advance p to the first vector that forms an angle >= 90 degrees (dot product <= 0)
            while p < base + n_vectors && doubled[base].dot(doubled[p]) > 0 {
                p += 1;
            }
            if q < p { q = p; }

            // Advance q to the first vector that forms an angle >= 180 degrees (cross product <= 0)
            while q < base + n_vectors && doubled[base].cross(doubled[q]) > 0 {
                q += 1;
            }

            // Range [p, q) contains all vectors forming 90 to 180 degrees with base
            if p < q {
                if doubled[base].dot(doubled[p]) == 0 {
                    right_triangles += 1;
                    obtuse_triangles += q - p - 1;
                } else {
                    obtuse_triangles += q - p;
                }
            }
        }
    }

    let total_triangles = n * (n - 1) * (n - 2) / 6;
    let acute_triangles = total_triangles - right_triangles - obtuse_triangles;
    
    println!("{} {} {}", acute_triangles, right_triangles, obtuse_triangles);
}
