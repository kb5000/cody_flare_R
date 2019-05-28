use super::super::utility::pos::Pos;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect {
    pub start: Pos,
    pub size: Pos,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line {
    pub start: Pos,
    pub size: Pos,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Triangle {
    pub a: Pos,
    pub b: Pos,
    pub c: Pos,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GeoObj {
    Rec(Rect), Lin(Line), Tri(Triangle)
}

impl Line {
}

impl Rect {
    pub fn pos_in(&self, pos: &Pos) -> bool {
        let end = self.start + self.size;
        (pos.x >= self.start.x && pos.x <= end.x) ||
            (pos.y >= self.start.y && pos.y <= end.y)
    }

    pub fn lines(&self) -> Vec<Line> {
        let mut res = Vec::new();
        let end = self.start + self.size;
        res.push(Line {start: self.start, size: Pos::new(self.size.x, 0.0)});
        res.push(Line {start: self.start, size: Pos::new(0.0, self.size.y)});
        res.push(Line {start: end, size: Pos::new(-self.size.x, 0.0)});
        res.push(Line {start: end, size: Pos::new(0.0, -self.size.y)});
        res
    }
}



