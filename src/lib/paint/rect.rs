use stretch::result::Layout;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl Rect {
    pub fn clip(&self, clip: &Self) -> Self {
        let x = clamp(clip.x, self.x, self.x + self.w);
        let y = clamp(clip.y, self.y, self.y + self.h);
        let w = clamp(clip.x + clip.w, self.x, self.x + self.w) - x;
        let h = clamp(clip.y + clip.h, self.y, self.y + self.h) - y;

        Rect { x, y, w, h }
    }
}

impl From<(u16, u16, u16, u16)> for Rect {
    fn from((x, y, w, h): (u16, u16, u16, u16)) -> Self {
        Self { x, y, w, h }
    }
}

impl From<Layout> for Rect {
    fn from(layout: Layout) -> Self {
        Self {
            x: layout.location.x.max(0.) as u16,
            y: layout.location.y.max(0.) as u16,
            w: layout.size.width as u16,
            h: layout.size.width as u16,
        }
    }
}

fn clamp(x: u16, min: u16, max: u16) -> u16 {
    x.max(min).min(max)
}

// ========================================================================= //
//                                   TESTS                                   //
// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn clamp() {
        assert_eq!(super::clamp(0, 1, 3), 1);
        assert_eq!(super::clamp(1, 1, 3), 1);
        assert_eq!(super::clamp(2, 1, 3), 2);
        assert_eq!(super::clamp(3, 1, 3), 3);
        assert_eq!(super::clamp(4, 1, 3), 3);
    }

    #[test]
    fn clip() {
        let rect = Rect {
            x: 1,
            y: 1,
            w: 3,
            h: 3,
        };
        assert_eq!(rect.clip(&rect), rect);

        let inside = Rect {
            x: 2,
            y: 2,
            w: 1,
            h: 1,
        };
        assert_eq!(rect.clip(&inside), inside);

        let outside = Rect {
            x: 0,
            y: 0,
            w: 5,
            h: 5,
        };
        assert_eq!(rect.clip(&outside), rect);

        let overlap = Rect {
            x: 2,
            y: 2,
            w: 5,
            h: 5,
        };
        assert_eq!(rect.clip(&overlap), Rect {
            x: 2,
            y: 2,
            w: 2,
            h: 2,
        });

        let overlap = Rect {
            x: 3,
            y: 0,
            w: 4,
            h: 2,
        };
        assert_eq!(rect.clip(&overlap), Rect {
            x: 3,
            y: 1,
            w: 1,
            h: 1,
        });
    }
}
