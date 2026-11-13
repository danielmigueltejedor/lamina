//! Resizing a window Lámina draws the frame of. With Nodalia's window controls the desktop draws
//! no border to grab, so the outer few points of the window take that job: the cursor turns into
//! a resize arrow there and a press hands the resize to the window manager.

use egui::{CursorIcon, Pos2, Rect, ResizeDirection, ViewportCommand};

/// How deep the resize zone reaches into the window, in points.
pub const EDGE: f32 = 6.0;

/// Corners are easier to hit with a larger zone along each side.
const CORNER: f32 = 16.0;

/// Which way a press at `p` resizes a window filling `screen`, if it is in an edge zone.
pub fn direction(screen: Rect, p: Pos2) -> Option<ResizeDirection> {
    if !screen.contains(p) {
        return None;
    }
    let (l, r) = (p.x - screen.min.x, screen.max.x - p.x);
    let (t, b) = (p.y - screen.min.y, screen.max.y - p.y);
    let near = |d: f32, reach: f32| d < reach;
    use ResizeDirection::*;
    let dir = if (near(l, EDGE) && near(t, CORNER)) || (near(t, EDGE) && near(l, CORNER)) {
        NorthWest
    } else if (near(r, EDGE) && near(t, CORNER)) || (near(t, EDGE) && near(r, CORNER)) {
        NorthEast
    } else if (near(l, EDGE) && near(b, CORNER)) || (near(b, EDGE) && near(l, CORNER)) {
        SouthWest
    } else if (near(r, EDGE) && near(b, CORNER)) || (near(b, EDGE) && near(r, CORNER)) {
        SouthEast
    } else if near(l, EDGE) {
        West
    } else if near(r, EDGE) {
        East
    } else if near(t, EDGE) {
        North
    } else if near(b, EDGE) {
        South
    } else {
        return None;
    };
    Some(dir)
}

fn cursor(dir: ResizeDirection) -> CursorIcon {
    use ResizeDirection::*;
    match dir {
        North => CursorIcon::ResizeNorth,
        South => CursorIcon::ResizeSouth,
        East => CursorIcon::ResizeEast,
        West => CursorIcon::ResizeWest,
        NorthEast => CursorIcon::ResizeNorthEast,
        NorthWest => CursorIcon::ResizeNorthWest,
        SouthEast => CursorIcon::ResizeSouthEast,
        SouthWest => CursorIcon::ResizeSouthWest,
    }
}

/// Shows the resize cursor over an edge and starts the window manager's resize on a press.
/// Does nothing while the window is maximized or fullscreen, where there is nothing to resize.
pub fn handle(ctx: &egui::Context) {
    let (screen, hover, pressed, fixed) = ctx.input(|i| {
        let v = i.viewport();
        (i.content_rect(), i.pointer.hover_pos(), i.pointer.primary_pressed(), v.maximized.unwrap_or(false) || v.fullscreen.unwrap_or(false))
    });
    if fixed {
        return;
    }
    let Some(dir) = hover.and_then(|p| direction(screen, p)) else { return };
    ctx.set_cursor_icon(cursor(dir));
    if pressed {
        ctx.send_viewport_cmd(ViewportCommand::BeginResize(dir));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::pos2;

    #[test]
    fn edges_and_corners_pick_their_direction() {
        let s = Rect::from_min_max(pos2(0.0, 0.0), pos2(800.0, 600.0));
        use ResizeDirection::*;
        assert_eq!(direction(s, pos2(2.0, 300.0)), Some(West));
        assert_eq!(direction(s, pos2(798.0, 300.0)), Some(East));
        assert_eq!(direction(s, pos2(400.0, 1.0)), Some(North));
        assert_eq!(direction(s, pos2(400.0, 599.0)), Some(South));
        assert_eq!(direction(s, pos2(2.0, 10.0)), Some(NorthWest));
        assert_eq!(direction(s, pos2(790.0, 598.0)), Some(SouthEast));
        assert_eq!(direction(s, pos2(400.0, 300.0)), None);
        assert_eq!(direction(s, pos2(-5.0, 300.0)), None);
    }
}
