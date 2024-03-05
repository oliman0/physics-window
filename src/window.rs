use std::ffi::CString;
use std::ptr;

use glfw::ffi::*;

pub struct Window {
    window: * mut GLFWwindow,
    // FPS & DELTA TIME
    last_time: f32,
    count_frames: i32,
    last_frame_time: f32,
    delta_time: f32,
    fps: i32,
    // WINDOW INFO
    window_size: nalgebra_glm::Vec2,
    frame_size: nalgebra_glm::Vec2,
    viewport_size: nalgebra_glm::Vec2,
    // INPUT
    cursor_locked: bool,
    keys: [bool; 1024],
    hold_keys: [bool; 1024],
    mouse_buttons: [bool; 12],
    hold_mouse_buttons: [bool; 12],
    mouse_pos: nalgebra_glm::Vec2,
    mouse_offset: nalgebra_glm::Vec2,
    scroll_wheel_x_offset: f32,
    scroll_wheel_y_offset: f32,
    sensitivity: f32
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            glfwTerminate();
        }
    }
}
impl Window {
    pub fn swap_buffers(&mut self) {
        unsafe { glfw::ffi::glfwSwapBuffers(self.window); }

        self.count_frames += 1;
        let current_time = unsafe { glfwGetTime() as f32 };

        if current_time - self.last_frame_time >= 1.0 {
            self.fps = self.count_frames;
            self.last_frame_time = current_time;
            self.count_frames = 0;
        }
    }
    pub fn should_close(&self) -> bool {unsafe { if glfwWindowShouldClose(self.window) == 1 {true} else {false}}}
    pub fn poll_events(&mut self) {
        self.mouse_offset = nalgebra_glm::vec2(0.0, 0.0); self.keys = [false; 1024]; self.mouse_buttons = [false; 12]; (self.scroll_wheel_x_offset, self.scroll_wheel_y_offset) = (0.0, 0.0);
        unsafe {glfwPollEvents();}
        
        // Handle Mouse Input
        let (mut xpos, mut ypos)= (0.0, 0.0);
        unsafe { glfwGetCursorPos(self.window, &mut xpos, &mut ypos) }
        self.mouse_pos = nalgebra_glm::vec2(xpos as f32, ypos as f32);

        let (center_x, center_y): (f32, f32) = (self.window_size.x as f32 / 2., self.window_size.y as f32 / 2.);

        self.mouse_offset = nalgebra_glm::vec2(xpos as f32 - center_x, center_y - ypos as f32);
        self.mouse_offset *= self.sensitivity * self.delta_time;

        if self.cursor_locked { unsafe { glfwSetCursorPos(self.window, self.window_size.x as f64 / 2.0, self.window_size.y as f64 / 2.0) } }
    }
    pub fn get_cursor_locked(&self) -> bool { self.cursor_locked }
    pub fn set_cursor_locked(&mut self, locked: bool) {
        unsafe {
            if locked { glfwSetInputMode(self.window, CURSOR, CURSOR_HIDDEN); }
            else { glfwSetInputMode(self.window, CURSOR, CURSOR_NORMAL); }
        }

        self.cursor_locked = locked;
    }
    pub fn get_deltatime(&mut self) -> f32 {
        let current_time = unsafe { glfwGetTime() as f32 };
        self.delta_time = current_time - self.last_time;
        self.last_time = current_time;

        self.delta_time
    }
    pub fn get_fps(&self) -> i32 { self.fps }

    pub fn get_key(&self, key: i32) -> bool { self.hold_keys[key as usize] }
    pub fn get_key_down(&self, key: i32) -> bool { self.keys[key as usize] }
    pub fn get_mouse_button(&self, button: i32) -> bool { self.hold_mouse_buttons[button as usize] }
    pub fn get_mouse_button_down(&self, button: i32) -> bool { self.mouse_buttons[button as usize] }
    pub fn get_mouse_position_raw(&self) -> nalgebra_glm::Vec2 { self.mouse_pos }
    pub fn get_mouse_position(&self) -> nalgebra_glm::Vec2 {
        nalgebra_glm::vec2(self.mouse_pos.x - (self.window_size.x - self.frame_size.x), 
        (self.window_size.y - self.mouse_pos.y) - (self.window_size.y - self.frame_size.y))
    }
    pub fn get_mouse_offset(&self) -> nalgebra_glm::Vec2 { self.mouse_offset }
    pub fn get_scroll_wheel_x_offset(&self) -> f32 { self.scroll_wheel_x_offset }
    pub fn get_scroll_wheel_y_offset(&self) -> f32 { self.scroll_wheel_y_offset }

    pub fn get_window_size(&self) -> nalgebra_glm::Vec2 { self.window_size }
    pub fn get_viewport_size(&self) -> nalgebra_glm::Vec2 { self.viewport_size } 

    pub fn get_window_position(&self) -> nalgebra_glm::Vec2 { unsafe { let (mut x, mut y): (i32, i32) = (0, 0); glfwGetWindowPos(self.window, &mut x, &mut y); nalgebra_glm::vec2(x as f32, y as f32) } }
    pub fn set_window_position(&mut self, pos: &nalgebra_glm::Vec2) { unsafe { glfwSetWindowPos(self.window, pos.x as i32, pos.y as i32) } }
}
pub fn window(title: &str, scr_width: i32, scr_height: i32, viewport_w: i32, viewport_h: i32, sensitivity: f32, d: i32) -> &mut Window {
    unsafe {

    // glfw: initialize and configure
    // ------------------------------
    if glfwInit() == 0 {
        println!("Failed to initialize GLFW");
        glfw::ffi::glfwTerminate();
    }
    //Setup GLFW window properties
    //OpenGL version
    glfwWindowHint(CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(CONTEXT_VERSION_MINOR, 3);
    // Core profile = No backwards compatibility
    glfwWindowHint(OPENGL_PROFILE, OPENGL_CORE_PROFILE);
    // Allow forward compatibility
    glfwWindowHint(OPENGL_FORWARD_COMPAT, 1);
    // Set Window non-resizable
    glfwWindowHint(RESIZABLE, 0);
    glfwWindowHint(DECORATED, d);

    // glfw window creation
    // --------------------
    let ctitle = CString::new(title.as_bytes()).unwrap();
    let glfwwindow = glfwCreateWindow(scr_width, scr_height, ctitle.as_ptr(), ptr::null_mut(), ptr::null_mut());

    let (mut fx, mut fy) = (0, 0);
    glfwGetFramebufferSize(glfwwindow, &mut fx, &mut fy);

    let window = Box::new(Window {
            window: glfwwindow,
            last_time: 0.0,
            delta_time: 0.0,
            count_frames: 0, last_frame_time: 0.0, fps: 0,
            window_size: nalgebra_glm::vec2(scr_width as f32, scr_height as f32), viewport_size: nalgebra_glm::vec2(viewport_w as f32, viewport_h as f32), frame_size: nalgebra_glm::vec2(fx as f32, fy as f32),
            cursor_locked: false,
            keys: [false; 1024],
            hold_keys: [false; 1024],
            mouse_buttons: [false; 12],
            hold_mouse_buttons: [false; 12],
            mouse_pos: nalgebra_glm::vec2(0.0, 0.0),
            mouse_offset: nalgebra_glm::vec2(0.0, 0.0),
            scroll_wheel_x_offset: 0.0,
            scroll_wheel_y_offset: 0.0,
            sensitivity: sensitivity
    });
    
    glfwMakeContextCurrent(glfwwindow);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|ptr| glfw::with_c_str(ptr, |ptr| glfwGetProcAddress(ptr)));

    glfwSetKeyCallback(glfwwindow, std::mem::transmute(key_callback as *const ()));
    glfwSetMouseButtonCallback(glfwwindow, std::mem::transmute(mouse_button_callback as *const ()));
    glfwSetScrollCallback(glfwwindow, std::mem::transmute(scroll_wheel_callback as *const ()));

    glfwSetWindowUserPointer(glfwwindow, std::mem::transmute(window));

    gl::Enable(gl::DEPTH_TEST);
    gl::DepthFunc(gl::LEQUAL);

    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

    glfwSwapInterval(1);
    
    //if glfwRawMouseMotionSupported() == 1 { glfwSetInputMode(glfwwindow, RAW_MOUSE_MOTION, 1); (&mut *(glfwGetWindowUserPointer(glfwwindow) as *mut Window)).sensitivity *= 0.3 }

    &mut *(glfwGetWindowUserPointer(glfwwindow) as *mut Window)
}
}

fn key_callback(window: *mut GLFWwindow, key: i32, _code: i32, action: i32, _mode: i32) {
    let window_user: &mut Window = unsafe { &mut *(glfwGetWindowUserPointer(window) as *mut Window) };

    if key == KEY_ESCAPE && action == PRESS {
        unsafe {
            glfwSetWindowShouldClose(window, 1);
        }
    }

    if key >= 0 && key < 1024
	{
		if action == PRESS
		{
			window_user.keys[key as usize] = true;
		    window_user.hold_keys[key as usize] = true;
		}
		else if action == RELEASE
		{
		    window_user.hold_keys[key as usize] = false;
		}
	}
}

fn mouse_button_callback(window: *mut GLFWwindow, button: i32, action: i32, _mods: i32) {
    let window_user: &mut Window = unsafe { &mut *(glfwGetWindowUserPointer(window) as *mut Window) };

    if button >= 0 && button < 12
	{
		if action == PRESS
		{
			window_user.mouse_buttons[button as usize] = true;
		    window_user.hold_mouse_buttons[button as usize] = true;
		}
		else if action == RELEASE
		{
		    window_user.hold_mouse_buttons[button as usize] = false;
		}
	}
}

fn scroll_wheel_callback(window: &mut GLFWwindow, xoff: f64, yoff: f64) {
    let window_user: &mut Window = unsafe { &mut *(glfwGetWindowUserPointer(window) as *mut Window) };

    window_user.scroll_wheel_x_offset = xoff as f32;
    window_user.scroll_wheel_y_offset = yoff as f32;
}