extern crate sdl2;
extern crate contracts;

pub mod rustyflame
{
pub mod render
{

use std::marker::Copy;

use sdl2::Sdl;
use sdl2::VideoSubsystem;
use sdl2::video::{ Window, WindowContext };
use sdl2::render::{ Canvas, Texture, TextureCreator };
use sdl2::pixels::PixelFormatEnum;

use contracts::ensures;

type RenderCanvas = Canvas<Window>;

#[derive(Copy, Clone)]
pub struct Dimensions
{
    pub width: u32,
    pub height: u32,
}

pub struct RenderConfig
{
    pub window_title: String,
    pub window_dimensions: Dimensions,
}

pub struct RenderTargetCreator
{
    window_dimensions: Dimensions,
    texture_creator: TextureCreator<WindowContext>,
}

impl RenderTargetCreator
{
    pub fn new(renderer: &Renderer) -> RenderTargetCreator
    {
        RenderTargetCreator 
        { 
            window_dimensions: renderer.config.window_dimensions,
            texture_creator: renderer.canvas.texture_creator(),
        }
    }

    pub fn create_render_target(&self, pixel_format: PixelFormatEnum, dimensions: &Dimensions) -> Result<Texture, String>
    {
        self.texture_creator
            .create_texture_streaming(pixel_format, dimensions.width, dimensions.height)
            .map_err(|e| e.to_string())
    }

    pub fn create_screen_render_target(&self, pixel_format: PixelFormatEnum) -> Result<Texture, String>
    {
        self.create_render_target(pixel_format, &self.window_dimensions)
    }
}


pub struct Renderer
{
    pub config: RenderConfig,
    video_subsystem: VideoSubsystem,
    pub canvas: RenderCanvas,
}

impl Renderer
{
    // -- INIT --

    pub fn new(sdl_context: &Sdl, config: RenderConfig) -> Result<Renderer, String>
    {
        let video_subsystem = Renderer::init_video_subsystem(&sdl_context)?;
        let canvas = Renderer::init_canvas(&video_subsystem, &config)?;

        Ok
        (
            Renderer 
            {
                config: config,
                video_subsystem: video_subsystem,
                canvas: canvas,
            }
        )
    }

    fn init_video_subsystem(sdl_context: &Sdl) -> Result<VideoSubsystem, String>
    {
        sdl_context.video()
    }

    fn init_sdl_window(video_subsystem: &VideoSubsystem, config: &RenderConfig) -> Result<Window, String>
    {
        video_subsystem
            .window(&config.window_title[..], config.window_dimensions.width, config.window_dimensions.height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
    }

    fn init_canvas(video_subsystem: &VideoSubsystem, config: &RenderConfig) -> Result<RenderCanvas, String>
    {
        let window = Renderer::init_sdl_window(&video_subsystem, &config)?;

        window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
    }

    // -- RENDER --

    pub fn clear(&mut self) 
    {
        self.canvas.clear()
    }

    pub fn submit(&mut self, texture: &Texture) -> Result<(), String>
    {
        self.canvas.copy(&texture, None, None)
    }

    pub fn render(&mut self)
    {
        self.canvas.present();
    }
}

}
}

use rustyflame::render::{ Renderer, RenderConfig, RenderTargetCreator, Dimensions };

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


fn main() -> Result<(), String> 
{
    let sdl_context = sdl2::init()?;

    let config = RenderConfig 
    {
        window_title: String::from("Flame renderer"),
        window_dimensions: Dimensions { width: 800, height: 600 },
    };

    let mut renderer = Renderer::new(&sdl_context, config)?;
    let render_target_creator = RenderTargetCreator::new(&renderer);

    let mut render_target = render_target_creator.create_screen_render_target(PixelFormatEnum::RGB24)?;

    render_target.with_lock(None, 
        |buffer: &mut [u8], pitch: usize| 
        {
            for y in 0..400
            {
                for x in 0..300
                {
                    let offset = y * pitch + x * 3;
                    buffer[offset] = x as u8;
                    buffer[offset + 1] = y as u8;
                    buffer[offset + 2] = 0;
                }
            }
        }
    )?;

    let mut evt_pump = sdl_context.event_pump()?;

    'mainloop: loop 
    {
        for evt in evt_pump.poll_iter()
        {
            match evt
            {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }

        renderer.clear();
        renderer.submit(&render_target)?;
        renderer.render();
    }

    Ok(())
}
