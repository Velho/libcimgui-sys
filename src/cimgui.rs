use bitflags::bitflags;

use crate::bindings::*; // ffi api

use sdl2::{event::Event, render::Canvas, video::Window};

bitflags! {
    #[derive (Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct IgNavInputFlags: i32 {
        const None = 0;
        const NavEnableKeyboard = 1;
        const NavEnableGamepad = 2;
        const NavEnableSetMousePos = 4;
        const NavNoCaptureKeyboard = 8;
        const NavNoMouse = 16;
        const NavNoMouseCursorChange = 32;
        const NavIsSRGB = 1048576;
        const NavIsTouchScreen = 2097152;
    }
}

pub enum IgStyle {
    Light,
    Dark,
}

pub fn show_demo_window(open: &mut bool) {
    unsafe {
        igShowDemoWindow(&mut *open);
    }
}

pub struct IgContext {
    ig_context: *mut ImGuiContext,
    config_flags: IgNavInputFlags,
}

impl IgContext {
    // CreateContext requires ImFontAtlas
    pub fn new() -> IgContext {
        unsafe {
            let font_atlas = std::ptr::null_mut(); // grab the atlas if needed
            let ig_context = igCreateContext(font_atlas);

            IgContext {
                ig_context,
                config_flags: IgNavInputFlags::None,
            }
        }
    }

    pub fn set_flags(&mut self, flags: IgNavInputFlags) {
        self.config_flags |= flags;
        unsafe {
            let mut io = igGetIO();
            (*io).ConfigFlags |= self.config_flags.bits();
        }
    }

    pub fn set_style(&self, style: IgStyle) {
        unsafe {
            let ig_style = igGetStyle();
            match style {
                IgStyle::Light => igStyleColorsLight(ig_style),
                IgStyle::Dark => igStyleColorsDark(ig_style),
            }
        }
    }

    pub fn new_frame(&self) {
        unsafe {
            igNewFrame();
        }
    }

    pub fn render(&self) {
        unsafe {
            igRender();
        }
    }
}

impl Drop for IgContext {
    fn drop(&mut self) {
        unsafe {
            igDestroyContext(self.ig_context);
        }
    }
}

pub struct IgDrawData {
    draw_data: *mut ImDrawData,
}

impl IgDrawData {
    pub fn get() -> IgDrawData {
        unsafe {
            let draw_data = igGetDrawData();

            IgDrawData { draw_data }
        }
    }
}

// SDL2Renderer does not really have a context in this case,
// the state is handled globally under the cimgui api
// perhaps the best way to implement this is to take
// the rust-sdl2 as a dependency and use it's abstractions properly
// _marker: PhantomData<&'a ()> ?
pub struct IgSDL2Renderer;

impl IgSDL2Renderer {
    pub fn new(canvas: &mut Canvas<Window>) -> IgSDL2Renderer {
        unsafe {
            let ret = ImGui_ImplSDL2_InitForSDLRenderer(
                canvas.window().raw() as *mut SDL_Window,
                canvas.raw() as *mut SDL_Renderer,
            );
            assert!(true == ret); // !panic

            let ret = ImGui_ImplSDLRenderer_Init(canvas.raw() as *mut SDL_Renderer);

            assert!(true == ret); // !panic
        }

        IgSDL2Renderer {} // return
    }

    fn _process_event(&self, evt: &mut sdl2::sys::SDL_Event) -> bool {
        unsafe {
            ImGui_ImplSDL2_ProcessEvent(std::mem::transmute(evt))
        }
    }

    /// Validates the event passed as parameter and,
    /// returns true if event was processed.
    ///
    /// # Arguments
    /// * `event` - Optional borrowed event
    ///
    /// Function is meant to be used as part of the SDL2
    /// event polling.
    /// # Example
    /// for event in sdl_context.event_pump()?.poll_iter()? {
    ///   renderer.process_events(Some(&event));
    ///   ...
    /// }
    /// Note: There might be Events which the to_ll can't
    /// convert in this case, we'll return false.
    pub fn process_events(&self, event: Option<&Event>) -> bool {
            if event.is_none() {
                return false;
            }

            let opt_event = &mut event.unwrap().to_ll();
            // couple of different events which rust sdl2 can't convert
            if opt_event.is_none() {
                return false;
            }
            // process the correct event
            self._process_event(&mut opt_event.unwrap())
    }

    pub fn new_frame(&self) {
        unsafe {
            ImGui_ImplSDLRenderer_NewFrame();
            ImGui_ImplSDL2_NewFrame();
        }
    }

    pub fn render_draw_data(&self, draw_data: IgDrawData) {
        unsafe {
            ImGui_ImplSDLRenderer_RenderDrawData(draw_data.draw_data);
        }
    }
}

impl Drop for IgSDL2Renderer {
    fn drop(&mut self) {
        unsafe {
            ImGui_ImplSDLRenderer_Shutdown();
            ImGui_ImplSDL2_Shutdown();
        }
    }
}
