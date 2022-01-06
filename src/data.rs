pub struct Gradient {
    pub(crate) name: String,
    pub(crate) segments: Vec<Segment>,
}

pub struct Segment {
    pub(crate) start: f64,
    pub(crate) handle_pos: f64,
    pub(crate) end: f64,
    pub(crate) start_color: RGBA,
    pub(crate) end_color: RGBA,
    pub(crate) kind: InterpolationKind,
}

#[derive(Clone, Copy)]
pub struct RGBA {
    pub(crate) r: f64,
    pub(crate) g: f64,
    pub(crate) b: f64,
    pub(crate) a: f64,
}

pub enum InterpolationKind {
    STEP
}

pub fn to_rgba(rgb :u32) -> RGBA {
    let r = (rgb >> 16) & 0xff;
    let g = (rgb >> 8) & 0xff;
    let b = rgb & 0xff;

    return RGBA {
        r: (r as f64 / 255.0),
        g: (g as f64 / 255.0),
        b: (b as f64 / 255.0),
        a: 1.0
    }
}

impl ToString for RGBA {
    fn to_string(&self) -> String {
        return format!("{} {} {} {}", self.r, self.g, self.b, self.a);
    }
}

impl ToString for InterpolationKind {
    fn to_string(&self) -> String {
        return match self {
            InterpolationKind::STEP => { "5 0 0 0".to_string() }
        };
    }
}

impl ToString for Segment {
    fn to_string(&self) -> String {
        return format!("{} {} {} {} {} {}", self.start, self.handle_pos, self.end, self.start_color.to_string(), self.end_color.to_string(), self.kind.to_string());
    }
}

impl ToString for Gradient {
    fn to_string(&self) -> String {
        let segs = &self.segments;
        let segcount = segs.len();
        let seg_str: String = segs.into_iter()
            .map(|seg| seg.to_string())
            .rfold(String::from(""), |acc, x| format!("{}\n{}", acc, x));

        return format!("GIMP Gradient\nName: {}\n{}{}", self.name, segcount, seg_str);
    }
}
