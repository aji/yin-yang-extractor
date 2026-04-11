use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Size<T> {
    pub w: T,
    pub h: T,
}

impl<T> From<(T, T)> for Size<T> {
    fn from((w, h): (T, T)) -> Self {
        Size { w, h }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rect<T> {
    pub x0: T,
    pub y0: T,
    pub x1: T,
    pub y1: T,
}

impl<T> From<(Point<T>, Point<T>)> for Rect<T> {
    fn from((p0, p1): (Point<T>, Point<T>)) -> Self {
        Rect {
            x0: p0.x,
            y0: p0.y,
            x1: p1.x,
            y1: p1.y,
        }
    }
}

impl<T: Clone + Add<Output = T>> From<(Point<T>, Size<T>)> for Rect<T> {
    fn from((p, sz): (Point<T>, Size<T>)) -> Self {
        Rect {
            x0: p.x.clone(),
            y0: p.y.clone(),
            x1: p.x + sz.w,
            y1: p.y + sz.h,
        }
    }
}
