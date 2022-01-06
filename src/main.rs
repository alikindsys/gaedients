use std::fs;
use crate::data::{Gradient, InterpolationKind, RGBA, Segment, to_rgba};
use crate::pride::{ read_pride_file};

mod data;
mod pride;

fn main() {
    for entry in fs::read_dir("./pride").unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            println!("[{:?}] Reading", entry.file_name());

            let state = read_pride_file(entry.path().to_str().unwrap());
            match state {
                None => {
                    println!("[{:?}] Failed.", entry.file_name());
                }
                Some(state) => {
                    let flag = Gradient {
                        name: state.name,
                        segments: make_segs(state.flag)
                    };
                    fs::create_dir_all("./pride/ggr/");
                    let ggr = format!("./pride/ggr/{}.ggr",entry.file_name().to_str().unwrap());
                    fs::write(&ggr,flag.to_string());
                    println!("[{:?}] Saved gradient into {}.", entry.file_name(),&ggr);
                }
            }
        }
    }
}

fn make_segs(colors: Vec<RGBA>) -> Vec<Segment> {
    let mut segs: Vec<Segment> = vec![];
    let inc = 1.0 / colors.len() as f64;
    let mid_point = inc / 2.0;

    let mut current = 0.0;

    for color in colors {
        segs.push(Segment {
            start: current,
            handle_pos: current + mid_point,
            end: current + inc,
            start_color: color,
            end_color: color,
            kind: InterpolationKind::STEP
        });

        current += inc;
    }

    // Patch fix for 0.999999... cases
    if segs.last().unwrap().end != 1.0 {
        let edit = segs.pop().unwrap();
        segs.push(Segment {
            start: edit.start,
            handle_pos: edit.handle_pos,
            end: 1.0,
            start_color: edit.start_color,
            end_color: edit.end_color,
            kind: edit.kind
        });
    }
    segs.sort_by(|x,y| y.start.partial_cmp(&x.start).unwrap());
    return segs
}
