use crate as blender;
use crate::blender::Blender;
use crate::rejection::{Reject, Rejection};
use std::convert::Infallible;

struct Input {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Input {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            w: 1,
            h: 1,
        }
    }
}

async fn blendf() -> Result<i32, Infallible> {
    Ok(5)
}

#[tokio::test]
async fn blend() {
    let input = Input::new();
    let blend = blender::from(input)
        .and(mouse_pos)
        .then(draw_mouse);
    // let blend = blender::any()
    //     .try_blend(blendf);
}
