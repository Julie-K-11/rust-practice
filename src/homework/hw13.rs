/*написати код:
    -який рахує зайняту площу
*/
struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}


fn area_occupied(rectangles: &Vec<Rectangle>) -> i32 {
    let mut x_coords: Vec<i32> = Vec::new();
    for rect in rectangles {
        x_coords.push(rect.a.x);
        x_coords.push(rect.b.x);
    }
    x_coords.sort();
    x_coords.dedup();

    let mut total_area = 0;
    for window in x_coords.windows(2) {
        let x1 = window[0];
        let x2 = window[1];
        let width = x2 - x1;
        if width <= 0 {
            continue;
        }

        let mut intervals: Vec<(i32, i32)> = Vec::new();
        for rect in rectangles {
            if rect.a.x <= x1 && rect.b.x >= x2 {
                intervals.push((rect.b.y, rect.a.y));
            }
        }

        if intervals.is_empty() {
            continue;
        }

        intervals.sort_by_key(|&(start, _)| start);
        let mut merged_start = intervals[0].0;
        let mut merged_end = intervals[0].1;
        let mut total_height = 0;
        for &(start, end) in intervals.iter().skip(1) {
            if start <= merged_end {
                if end > merged_end {
                    merged_end = end;
                }
            } else {
                total_height += merged_end - merged_start;
                merged_start = start;
                merged_end = end;
            }
        }
        total_height += merged_end - merged_start;
        total_area += width * total_height;
    }
    total_area
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

#[test]
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}