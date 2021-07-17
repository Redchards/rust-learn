use rand::Rng;
use sdl2::pixels::PixelFormatEnum;

use super::render::{ Dimensions, Renderer, RenderTargetCreator };
use super::math::{ AugmentedMat2x2, Vec2D };

struct State
{
    current_coordinates: Vec2D,
    density: Vec<usize>,
    plot_dimensions: Dimensions,
    iter_count: usize,
}

impl State
{
    fn new(plot_dimensions: &Dimensions) -> State 
    {
        let mut thread_rng = rand::thread_rng();
        
        State
        {
            current_coordinates: Vec2D{ x: thread_rng.gen_range(-1.0..1.0), y: thread_rng.gen_range(-1.0..1.0) },
            density: vec![0; (plot_dimensions.height * plot_dimensions.width) as usize],
            plot_dimensions: plot_dimensions.clone(),
            iter_count: 0,
        }
    }

    fn plot_current(&mut self)
    {
        let height = self.plot_dimensions.height as f64;
        let width = self.plot_dimensions.width as f64;

        let x = ((self.current_coordinates.x + 1.) * height / 2.) as usize;
        let y = ((self.current_coordinates.y + 1.) * width / 2.) as usize;

        let idx = x * width as usize + y;

        if idx > 0 && idx < self.density.len()
        {
            self.density[idx] += 1;
        }

    }
}

pub struct Simulator
{
    state: State,
    renderer: Renderer,
    render_target_creator: RenderTargetCreator,
    preparation_rounds: usize,
}

impl Simulator
{
	pub fn new(renderer: Renderer, preparation_rounds: usize) -> Result<Simulator, String>
	{
        let render_target_creator = RenderTargetCreator::new(&renderer);

        Ok(Simulator 
        { 
            state: State::new(&renderer.config.window_dimensions),
            renderer: renderer,
            render_target_creator: render_target_creator,
            preparation_rounds: preparation_rounds,
        })
    }

    pub fn reset(&mut self)
    {
        self.state = State::new(&self.renderer.config.window_dimensions);
    }

    pub fn step(&mut self)
    {
        let fs = vec![
            AugmentedMat2x2::new(
                [[1./2., 0., 0.],
                 [0., 1./2., 0.]]
            ),
            AugmentedMat2x2::new(
                [[1./2., 0., 1./2.],
                 [0., 1./2., 0.]]
            ),
            AugmentedMat2x2::new(
                [[1./2., 0., 0.],
                 [0., 1./2., 1./2.]]
            )
        ];

        self.state.iter_count += 1;
        let choice = rand::thread_rng().gen_range(0..fs.len());
        let application = fs[choice];

        self.state.current_coordinates = application * self.state.current_coordinates;

        if self.state.iter_count > self.preparation_rounds
        {
            self.state.plot_current();
        }

    }

    pub fn render(&mut self) -> Result<(), String>
    {
        let mut render_target = self.render_target_creator.create_screen_render_target(PixelFormatEnum::ARGB8888)?;

        render_target.with_lock(None,
            |buffer: &mut [u8], _: usize|
            {
                for (idx, d) in self.state.density.iter().enumerate()
                {
                    if *d > 0
                    {
                        let offset = idx * 4;

                        buffer[offset]     = 0xFF;
                        buffer[offset + 1] = 0xFF;
                        buffer[offset + 2] = 0xFF;
                        buffer[offset + 3] = 0xFF;
                    }
               }
            }
        )?;


        self.renderer.clear();
        self.renderer.submit(&render_target)?;
        self.renderer.render();

        Ok(())
    }
}