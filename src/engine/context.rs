use std::marker::PhantomData;
use sdl2::{
    Sdl,
    video::Window
};
use {
    resources::Cursor
};

pub struct Context<'a> {
    pub sdl_context: Sdl,
    pub window: Window,

    pub cursor: Cursor,

    pub _marker: PhantomData<&'a ()>
}