extern crate sdl2;
extern crate contracts;

mod rustyflame;

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

    let mut render_target = render_target_creator.create_screen_render_target(PixelFormatEnum::ARGB8888)?;

    println!("{:?}", renderer.get_info().texture_formats);

    render_target.with_lock(None, 
        |buffer: &mut [u8], pitch: usize| 
        {
            for y in 0..400
            {
                for x in 0..300
                {
                    let offset = y * pitch + x * 4;
                    buffer[offset + 1] = x as u8;
                    buffer[offset + 2] = y as u8;
                    buffer[offset + 3] = 0;
                    buffer[offset] = 0;
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
