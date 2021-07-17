extern crate sdl2;
extern crate contracts;

mod rustyflame;

use rustyflame::render::{ Renderer, RenderConfig, Dimensions };
use rustyflame::simulator::{ Simulator };

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

    let renderer = Renderer::new(&sdl_context, config)?;
    let mut simulator = Simulator::new(renderer, 50)?;
    let mut evt_pump = sdl_context.event_pump()?;

    let mut iter = 0usize;

    'mainloop: loop 
    {
        for evt in evt_pump.poll_iter()
        {
            match evt
            {
                Event::Quit { .. }
                | Event::KeyDown 
                {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                Event::KeyDown
                {
                    keycode: Some(Keycode::R),
                    ..
                } => simulator.reset(),
                _ => {}
            }
        }

        iter += 1;

        simulator.step();

        if iter%1000 == 0
        {
            simulator.render()?;
        }
    }

    Ok(())
}
