use lazy_static::lazy_static;
use resources::{Cursor, WindowSize};
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::{ttf::Sdl2TtfContext, video::Window, Sdl};
use sdl2_extras::{
    adapters::{CanvasAdapter, ResourceFacade},
    common::GameTime,
};
use specs::World;
use std::error::Error;
use std::sync::Mutex;
use managers::EventManager;

struct SdlInitializationContext(Sdl, Sdl2TtfContext);
unsafe impl Send for SdlInitializationContext {}
unsafe impl Sync for SdlInitializationContext {}

struct GameContext(
    Mutex<Option<WindowSize>>,
    Mutex<Option<Canvas<Window>>>,
    TextureCreator<WindowContext>,
);
unsafe impl Send for GameContext {}
unsafe impl Sync for GameContext {}

pub struct InitializationContext(
    CanvasAdapter,
    WindowSize,
    EventManager,
    Cursor,
    ResourceFacade<'static>,
);

lazy_static! {
    static ref sdl_contexts: SdlInitializationContext = initialize_sdl().unwrap();
    static ref game_context: GameContext = initialize_game().unwrap();
}

fn initialize_sdl() -> Result<SdlInitializationContext, Box<Error>> {
    let sdl_context = sdl2::init()?;
    let font_context = sdl2::ttf::init()?;

    Ok(SdlInitializationContext(sdl_context, font_context))
}

fn initialize_game() -> Result<GameContext, Box<Error>> {
    let window = create_window(&sdl_contexts.0)?;
    let window_size = window.size();
    let canvas = window.into_canvas().build()?;
    let texture_creator = canvas.texture_creator();

    Ok(GameContext(
        Mutex::new(Some(WindowSize(window_size))),
        Mutex::new(Some(canvas)),
        texture_creator,
    ))
}

pub fn initialize() -> Result<InitializationContext, Box<Error>> {
    let SdlInitializationContext(ref sdl_context, ref font_context) = *sdl_contexts;
    let GameContext(ref window_size, ref canvas, ref texture_creator) = *game_context;
    
    let window_size = window_size.lock().unwrap().take().unwrap();
    let canvas = canvas.lock().unwrap().take();

    let canvas_adapter = CanvasAdapter::new(canvas);
    let event_manager = EventManager::new(&sdl_context)?;

    let mut cursor = Cursor::new(sdl_context.mouse());
    cursor.hide_system();

    let resource_facade = ResourceFacade::new(&font_context, &texture_creator);

    Ok(InitializationContext(
        canvas_adapter,
        window_size,
        event_manager,
        cursor,
        resource_facade,
    ))
}

pub fn create_world(context: InitializationContext) -> Result<World, Box<Error>> {
    let InitializationContext(
        canvas_adapter,
        windows_size,
        event_manager,
        cursor,
        resource_facade
    ) = context;

    let mut world = World::new();

    world.add_resource(canvas_adapter);
    world.add_resource(windows_size);
    world.add_resource(event_manager);
    world.add_resource(cursor);
    world.add_resource(resource_facade);
    world.add_resource(GameTime::default());

    Ok(world)
}

fn create_window(sdl_context: &Sdl) -> Result<Window, Box<Error>> {
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rust demo", 800, 600)
        .position_centered()
        .build()?;

    Ok(window)
}
