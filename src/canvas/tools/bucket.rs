use crate::{canvas::Canvas, util::*};

impl Canvas {
    pub fn bucket(&mut self, point: Point, color: Color) {
        let first_color = self.get_color(point);
        if first_color == color {
            return;
        }
        let mut points = Vec::<Point>::new();
        let mut new_points = Vec::<Point>::new();
        new_points.push(point);

        while new_points.len() != 0 {
            let points_to_be_processed = new_points.clone();
            points.append(&mut new_points);
            for mut point in points_to_be_processed {
                let current_point = point;

                if point.x != 0 {
                    point.x -= 1;
                    self.process_point(point, first_color, color, &mut new_points);
                    point = current_point;
                }

                if point.x != self.terminal.size.width {
                    point.x += 1;
                    self.process_point(point, first_color, color, &mut new_points);
                    point = current_point;
                }

                if point.y != 0 {
                    point.y -= 1;
                    self.process_point(point, first_color, color, &mut new_points);
                    point = current_point;
                }

                if point.y != self.terminal.size.height * 2 {
                    point.y += 1;
                    self.process_point(point, first_color, color, &mut new_points);
                }
            }
        }
    }

    fn process_point(
        &mut self,
        point: Point,
        first_color: Color,
        color: Color,
        new_points: &mut Vec<Point>,
    ) {
        if self.get_color(point) == first_color {
            self.block(point, color);
            new_points.push(point);
        }
    }
}
