use rand::Rng;
use sdl2::pixels::{ PixelFormatEnum, Color };

use super::render::{ Dimensions, Renderer, RenderTargetCreator };
use super::math::{ AugmentedMat2x2, Vec2D };
use super::utils;

fn generate_random_rgb() -> Color 
{
    let mut rnd = rand::thread_rng();

    Color::RGB
    ( 
        rnd.gen_range(0..255),
        rnd.gen_range(0..255),
        rnd.gen_range(0..255)
    )
}

fn mix_rgb(c1: Color, c2: Color, factor: f64) -> Color
{
    Color::RGB
    (
        ((u8::max(c1.r, c2.r) - u8::min(c1.r, c2.r)) as f64 * factor) as u8 + u8::min(c1.r, c2.r),
        ((u8::max(c1.g, c2.g) - u8::min(c1.g, c2.g)) as f64 * factor) as u8 + u8::min(c1.g, c2.g),
        ((u8::max(c1.b, c2.b) - u8::min(c1.b, c2.b)) as f64 * factor) as u8 + u8::min(c1.b, c2.b),
    )
}

#[derive(Clone, Copy)]
pub enum SuperSamplingFactor
{
    None = 1,
    X2 = 2,
    X4 = 4,
    X8 = 8,
}

struct State
{
    current_coordinates: Vec2D,
    current_color: Color,
    density_histogram: Vec<usize>,
    color_histogram: Vec<Color>,
    plot_dimensions: Dimensions,
    iter_count: usize,
    super_sampling_factor: SuperSamplingFactor,
}

impl State
{
    fn new(render_target_dimensions: &Dimensions, super_sampling_factor: SuperSamplingFactor) -> State 
    {
        let mut thread_rng = rand::thread_rng();

        let plot_dimensions = Dimensions
        { 
            height: render_target_dimensions.height * super_sampling_factor as u32, 
            width: render_target_dimensions.width * super_sampling_factor as u32, 
        };
        
        let factor = super_sampling_factor as usize;
        
        State
        {
            current_coordinates: Vec2D{ x: thread_rng.gen_range(-1.0..1.0), y: thread_rng.gen_range(-1.0..1.0) },
            current_color: generate_random_rgb(),
            density_histogram: vec![0; plot_dimensions.height as usize * factor * plot_dimensions.width as usize * factor],
            color_histogram: vec![Color::RGB(0, 0, 0); plot_dimensions.height as usize * factor * plot_dimensions.width as usize * factor],
            plot_dimensions: plot_dimensions,
            iter_count: 0,
            super_sampling_factor: super_sampling_factor,
        }
    }

    fn plot_current(&mut self)
    {
        let height = self.plot_dimensions.height as f64;
        let width = self.plot_dimensions.width as f64;

        let x = ((self.current_coordinates.x + 1.) * height / 2.) as usize;
        let y = ((self.current_coordinates.y + 1.) * width / 2.) as usize;

        if x < self.plot_dimensions.height as usize && y < self.plot_dimensions.width as usize
        {
            let idx = x * width as usize + y;

            if idx > 0 && idx < self.density_histogram.len()
            {
                self.density_histogram[idx] += 1;
                self.color_histogram[idx] = self.current_color;
            }
        }
    }

    fn compute_subsampled_histogram(&self) -> (Vec<usize>, Vec<Color>)
    {
        let subsampled_dims = Dimensions 
        { 
            height: self.plot_dimensions.height / self.super_sampling_factor as u32, 
            width: self.plot_dimensions.width / self.super_sampling_factor as u32,
        };

        let mut histogram = vec![0; (subsampled_dims.height * subsampled_dims.width) as usize];
        let mut color_histogram = vec![Color::RGB(0, 0, 0); (subsampled_dims.height * subsampled_dims.width) as usize];
        let factor = self.super_sampling_factor as u32;

        for x in 0..subsampled_dims.height
        {
            for y in 0..subsampled_dims.width
            {
                let idx = (x * factor * subsampled_dims.width) + (y * factor);

                let mut val = 0;
                let mut area_colors = vec![];

                for offset_x in 0..factor
                {
                    for offset_y in 0..factor
                    {
                        if x + offset_x < self.plot_dimensions.height && y + offset_y < self.plot_dimensions.width
                        {
                            let offset_idx = (idx + offset_x * self.plot_dimensions.width + offset_y) as usize;

                            val += self.density_histogram[offset_idx];
                            area_colors.push(self.color_histogram[offset_idx]);
                        }
                    }
                }

                let subsampled_hist_idx = (x * subsampled_dims.width + y) as usize;
                histogram[subsampled_hist_idx] = val;

                let mut float_rgb = (0., 0., 0.);

                for c in area_colors.iter()
                {
                    float_rgb.0 += c.r as f64;
                    float_rgb.1 += c.g as f64;
                    float_rgb.2 += c.b as f64;
                }

                color_histogram[subsampled_hist_idx] = Color::RGB
                (
                    (float_rgb.0 / area_colors.len() as f64) as u8, 
                    (float_rgb.1 / area_colors.len() as f64) as u8, 
                    (float_rgb.2 / area_colors.len() as f64) as u8,
                );
            }
        }

        (histogram, color_histogram)
    }
}

struct FunctionElement
{
    application: AugmentedMat2x2,
    associated_color: Color,
    
}

pub struct Simulator
{
    state: State,
    renderer: Renderer,
    render_target_creator: RenderTargetCreator,
    preparation_rounds: usize,
    ifs: Vec<FunctionElement>,
    super_sampling_factor: SuperSamplingFactor,
}

impl Simulator
{
	pub fn new(renderer: Renderer, preparation_rounds: usize, super_sampling_factor: SuperSamplingFactor) -> Result<Simulator, String>
	{
        let render_target_creator = RenderTargetCreator::new(&renderer);

        Ok(Simulator 
        { 
            state: State::new(&renderer.config.window_dimensions, super_sampling_factor),
            renderer: renderer,
            render_target_creator: render_target_creator,
            preparation_rounds: preparation_rounds,
            ifs: Simulator::generate_norm_ifs(),
            super_sampling_factor: super_sampling_factor,
        })
    }

    fn generate_ifs() -> Vec<FunctionElement>
    {
        vec![
            FunctionElement{ application: AugmentedMat2x2::rand(-1., 1.), associated_color: generate_random_rgb() },
            FunctionElement{ application: AugmentedMat2x2::rand(-1., 1.), associated_color: generate_random_rgb() },
            FunctionElement{ application: AugmentedMat2x2::rand(-1., 1.), associated_color: generate_random_rgb() },
        ]
    }

    fn generate_ifs2() -> Vec<FunctionElement>
    {
        vec![
            FunctionElement{ application: AugmentedMat2x2::rand(-1., 1.), associated_color: generate_random_rgb() },
            FunctionElement{ application: AugmentedMat2x2::rand(-1., 1.), associated_color: generate_random_rgb() },
            FunctionElement{ application: AugmentedMat2x2::rand(-1., 1.), associated_color: generate_random_rgb() },
            FunctionElement{ application: AugmentedMat2x2::rand(-1., 1.), associated_color: generate_random_rgb() },
            FunctionElement{ application: AugmentedMat2x2::rand(-1., 1.), associated_color: generate_random_rgb() },
            FunctionElement{ application: AugmentedMat2x2::rand(-1., 1.), associated_color: generate_random_rgb() },
            FunctionElement{ application: AugmentedMat2x2::rand(-1., 1.), associated_color: generate_random_rgb() },
            FunctionElement{ application: AugmentedMat2x2::rand(-1., 1.), associated_color: generate_random_rgb() },
            FunctionElement{ application: AugmentedMat2x2::rand(-1., 1.), associated_color: generate_random_rgb() },
            FunctionElement{ application: AugmentedMat2x2::rand(-1., 1.), associated_color: generate_random_rgb() },
            FunctionElement{ application: AugmentedMat2x2::rand(-1., 1.), associated_color: generate_random_rgb() },
        ]
    }

    fn generate_norm_ifs() -> Vec<FunctionElement>
    {
        vec![
            FunctionElement{ application: AugmentedMat2x2::rand_std(), associated_color: generate_random_rgb() },
            FunctionElement{ application: AugmentedMat2x2::rand(-1., 1.), associated_color: generate_random_rgb() },
            FunctionElement{ application: AugmentedMat2x2::rand_std(), associated_color: generate_random_rgb() },
        ]
    }

    pub fn reset(&mut self)
    {
        self.state = State::new(&self.renderer.config.window_dimensions, self.super_sampling_factor);
        self.ifs = Simulator::generate_ifs2();
    }

    pub fn step(&mut self)
    {
        /*let fs = vec![
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
        ];*/

        self.state.iter_count += 1;
        let choice = rand::thread_rng().gen_range(0..self.ifs.len());
        let FunctionElement{ application, associated_color} = self.ifs[choice];

        self.state.current_coordinates = application * self.state.current_coordinates;
        self.state.current_color = mix_rgb(self.state.current_color, associated_color, 0.5);

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
                let (histogram, colors) = self.state.compute_subsampled_histogram();
                let max_density = *histogram.iter().max().unwrap();
                let max_log_density = (max_density as f64).log10();

                for (idx, (d, c)) in utils::zip((histogram.iter(), colors)).enumerate()
                {
                    if *d > 0
                    {
                        let gamma_factor = 1./2.2;
                        let r = (c.r as f64 * ((*d as f64).log10() / max_log_density).powf(gamma_factor)) as u8;
                        let g = (c.g as f64 * ((*d as f64).log10() / max_log_density).powf(gamma_factor)) as u8;
                        let b = (c.b as f64 * ((*d as f64).log10() / max_log_density).powf(gamma_factor)) as u8;

                        let offset = idx * 4;

                        buffer[offset]     = r;
                        buffer[offset + 1] = g;
                        buffer[offset + 2] = b;
                        buffer[offset + 3] = 0;
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