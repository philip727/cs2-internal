use hudhook::imgui::Ui;
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

use super::view_matrix::ViewMatrix4x4;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl WorldToScreen for Vector3D {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn z(&self) -> f32 {
        self.z
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(align(16))]
pub struct VectorAligned3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub trait WorldToScreen {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;

    fn world_to_screen(&self, matrix: &ViewMatrix4x4, imgui_ui: &Ui) -> Option<Vector2D> {
        let matrix = matrix.matrix;
        let width = matrix[3][0] * self.x()
            + matrix[3][1] * self.y()
            + matrix[3][2] * self.z()
            + matrix[3][3];

        if width < 0.001 {
            return None;
        }

        let w_inverse = 1f32 / width;

        let mut d_x = matrix[0][0] * self.x()
            + matrix[0][1] * self.y()
            + matrix[0][2] * self.z()
            + matrix[0][3];
        d_x *= w_inverse;

        let mut d_y = matrix[1][0] * self.x()
            + matrix[1][1] * self.y()
            + matrix[1][2] * self.z()
            + matrix[1][3];
        d_y *= w_inverse;

       // println!("{d_x}, {d_y}");

        let display_size = imgui_ui.io().display_size;

        let mut screen_x = display_size[0] * 0.5f32;
        let mut screen_y = display_size[1] * 0.5f32;

        screen_x += 0.5f32 * d_x * display_size[0] + 0.5f32;
        screen_y -= 0.5f32 * d_y * display_size[1] + 0.5f32;

        Some(Vector2D {
            x: screen_x,
            y: screen_y,
        })
        //let matrix = matrix.matrix;
        //let mut d_x = matrix[0][0] * self.x()
        //    + matrix[0][1] * self.y()
        //    + matrix[0][2] * self.z()
        //    + matrix[0][3];

        //let mut d_y = matrix[1][0] * self.x()
        //    + matrix[1][1] * self.y()
        //    + matrix[1][2] * self.z()
        //    + matrix[1][3];

        //let w = matrix[3][0] * self.x()
        //    + matrix[3][1] * self.y()
        //    + matrix[3][2] * self.z()
        //    + matrix[3][3];

        //let inverted_w = 1.0 / w;
        //d_x *= inverted_w;
        //d_y *= inverted_w;

        //unsafe {
        //    let mut screen_x = (GetSystemMetrics(SM_CXSCREEN) as f32) * 0.5;
        //    let mut screen_y = (GetSystemMetrics(SM_CYSCREEN) as f32) * 0.5;

        //    screen_x += 0.5 * d_x * (GetSystemMetrics(SM_CXSCREEN) as f32) + 0.5;
        //    screen_y -= 0.5 * d_y * (GetSystemMetrics(SM_CYSCREEN) as f32) + 0.5;

        //    Vector3D {
        //        x: screen_x,
        //        y: screen_y,
        //        z: w,
        //    }
        //}
    }
}
