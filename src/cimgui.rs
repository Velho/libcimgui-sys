use std::ptr;
use bitflags::bitflags;

use crate::bindings::*; // ffi api

use sdl2::*;

bitflags::bitflags! {
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
    Dark
}

pub struct IgContext {
    ig_context: *mut ImGuiContext,
    config_flags: IgNavInputFlags
}

impl IgContext {
    // CreateContext requires ImFontAtlas
    pub fn new() -> IgContext {
        unsafe {
            let font_atlas = std::ptr::null_mut(); // grab the atlas if needed
            let ig_context = igCreateContext(font_atlas);

            IgContext { ig_context, config_flags: IgNavInputFlags::None }
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
                IgStyle::Dark => igStyleColorsDark(ig_style)
            }
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

// SDL2Renderer does not really have a context in this case,
// the state is handled globally under the cimgui api
// perhaps the bst way to implement this is to take
// the rust-sdl2 as a dependency and use it's abstractions properly
pub struct IgSDL2Renderer;

impl IgSDL2Renderer {
    // looks a bit unsafe interface thoo
    pub fn new(window: *mut SDL_Window, canvas: *mut SDL_Renderer) -> IgSDL2Renderer {
    // pub fn new(window: ) -> IgSDL2Renderer {
        unsafe {
            let ret = ImGui_ImplSDL2_InitForSDLRenderer(window, canvas);
            assert!(true == ret); // !panic

            let ret = ImGui_ImplSDLRenderer_Init(canvas);
            assert!(true == ret); // !panic
        }

        IgSDL2Renderer {}
    }

}
