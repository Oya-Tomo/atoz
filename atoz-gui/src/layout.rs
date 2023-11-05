// # Layout Calculation
// ## description
//     simple, orignal, calculation method to solve UI layout problem
// ## calc steps
//     1. subtract all sizes of constant areas from parent size.
//     2. subtract all sizes of areas constrained by percentage from remained area.
//     3. give permissions for rendering to widgets.
// ## if the parent size is smaller than self.min
//     return 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub left: usize,
    pub top: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(left: usize, top: usize, width: usize, height: usize) -> Self {
        return Self {
            left,
            top,
            width,
            height,
        };
    }

    pub fn left(&self) -> usize {
        return self.left;
    }

    pub fn right(&self) -> usize {
        return self.left + self.width;
    }

    pub fn top(&self) -> usize {
        return self.top;
    }

    pub fn bottom(&self) -> usize {
        return self.top + self.height;
    }

    pub fn width(&self) -> usize {
        return self.width;
    }

    pub fn height(&self) -> usize {
        return self.height;
    }

    pub fn subtract_padding(&self, padding: Padding) -> Rect {
        let mut rect = self.clone();

        let left = padding.left.calculate(self.width(), self.width());
        let right = padding.right.calculate(self.width(), self.width());
        let top = padding.top.calculate(self.height(), self.height());
        let bottom = padding.bottom.calculate(self.height(), self.height());

        let width = rect.width() as isize - left as isize - right as isize;
        let height = rect.height() as isize - top as isize - bottom as isize;

        if width > 0 {
            rect.left += left;
            rect.width = width as usize;
        } else {
            rect.left += self.width() / 2;
            rect.width = 0;
        }
        if height > 0 {
            rect.top += top;
            rect.height = height as usize;
        } else {
            rect.top += self.height() / 2;
            rect.height = 0;
        }
        return rect;
    }
}

impl Default for Rect {
    fn default() -> Self {
        return Self {
            left: 0,
            top: 0,
            width: 0,
            height: 0,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Pixel(usize),
    Percent(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Constraint {
    size: Unit,
    max: usize,
    min: usize,
}

impl Constraint {
    pub fn pixel(size: usize, min: usize) -> Self {
        return Self {
            size: Unit::Pixel(size),
            max: size,
            min,
        };
    }

    pub fn percent(percent: usize, max: usize, min: usize) -> Self {
        return Self {
            size: Unit::Percent(percent),
            max,
            min,
        };
    }

    pub fn calculate(&self, parent: usize, space: usize) -> usize {
        match self.size {
            Unit::Pixel(pixel) => {
                if pixel <= space {
                    return pixel;
                } else if self.min <= space {
                    return space;
                } else {
                    return 0;
                }
            }
            Unit::Percent(percent) => {
                let mut size = parent * percent / 100;
                if size > self.max {
                    size = self.max;
                } else if size < self.min {
                    size = self.min;
                }

                if size <= space {
                    return size;
                } else if self.min <= space {
                    return space;
                } else {
                    return 0;
                }
            }
        };
    }

    pub fn has_pixel(&self) -> bool {
        return match self.size {
            Unit::Pixel(_) => true,
            _ => false,
        };
    }

    pub fn has_percent(&self) -> bool {
        return match self.size {
            Unit::Percent(_) => true,
            _ => false,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Start,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Padding {
    pub left: Constraint,
    pub right: Constraint,
    pub top: Constraint,
    pub bottom: Constraint,
}

impl Default for Padding {
    fn default() -> Self {
        return Self {
            left: Constraint::pixel(0, 0),
            right: Constraint::pixel(0, 0),
            top: Constraint::pixel(0, 0),
            bottom: Constraint::pixel(0, 0),
        };
    }
}

#[cfg(test)]
mod test {
    use super::{Constraint, Padding, Rect};

    #[test]
    fn rect_reshape() {
        let rect = Rect::new(100, 100, 100, 100).subtract_padding(Padding {
            left: Constraint::percent(60, 100, 0),
            right: Constraint::percent(50, 100, 0),
            top: Constraint::pixel(20, 0),
            bottom: Constraint::pixel(20, 0),
        });
        println!("{:?}", rect);
    }
}
