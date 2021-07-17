use std::ops::{ Add, Sub, Mul };
use std::cmp::{ PartialEq };

use rand;
use rand::Rng;
use rand_distr::{ Normal, Distribution };

use super::utils;


#[derive(Clone, Copy, Debug)]
pub struct Vec2D
{
	pub x: f64,
    pub y: f64
}

impl Add for Vec2D
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output 
    {
        Self
        {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2D
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output
    {
        Self
        {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl PartialEq for Vec2D
{
    fn eq(&self, other: &Self) -> bool
    {
        self.x == other.x
        && self.y == other.y
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AugmentedMat2x2
{
    coeffs: [[f64; 3]; 2],
}

impl AugmentedMat2x2
{
    pub fn new(coeffs: [[f64; 3]; 2]) -> AugmentedMat2x2
    {
        AugmentedMat2x2 { coeffs: coeffs }
    }

    pub fn rand(min: f64, max: f64) -> AugmentedMat2x2
    {
        let mut rnd = rand::thread_rng();

        AugmentedMat2x2::new
        (
            [[rnd.gen_range(min..max), rnd.gen_range(min..max), rnd.gen_range(min..max)],
             [rnd.gen_range(min..max), rnd.gen_range(min..max), rnd.gen_range(min..max)]] 
        )
    }

    pub fn rand_std() -> AugmentedMat2x2
    {
        let mut rnd = rand::thread_rng();
        let distrib = Normal::new(0.0, 0.5).unwrap();

        AugmentedMat2x2::new
        (
            [[rnd.sample(distrib), rnd.sample(distrib), rnd.sample(distrib)],
             [rnd.sample(distrib), rnd.sample(distrib), rnd.sample(distrib)]]
        )
    }
}

impl Mul<Vec2D> for AugmentedMat2x2
{
    type Output = Vec2D;

    fn mul(self, other: Vec2D) -> Self::Output
    {
        Self::Output
        {
            x: self.coeffs[0][0] * other.x + self.coeffs[0][1] * other.y + self.coeffs[0][2],
            y: self.coeffs[1][0] * other.x + self.coeffs[1][1] * other.y + self.coeffs[1][2],
        }
    }
}

impl Mul<f64> for AugmentedMat2x2
{
    type Output = AugmentedMat2x2;

    fn mul(self, scalar: f64) -> Self::Output
    {
        Self
        {
            coeffs: 
            [
                [
                    scalar * self.coeffs[0][0],
                    scalar * self.coeffs[0][1],
                    scalar * self.coeffs[0][2],
                ],
                [
                    scalar * self.coeffs[1][0],
                    scalar * self.coeffs[1][1],
                    scalar * self.coeffs[1][2],
                ],
            ]
        }
    }
}

impl Mul<AugmentedMat2x2> for f64
{
    type Output = AugmentedMat2x2;

    fn mul(self, mat: AugmentedMat2x2) -> Self::Output
    {
        mat * self
    }
}


impl Add for AugmentedMat2x2
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output
    {
        Self
        {
            coeffs: 
            [
                [
                    self.coeffs[0][0] + other.coeffs[0][0],
                    self.coeffs[0][1] + other.coeffs[0][1],
                    self.coeffs[0][2] + other.coeffs[0][2],
                ],
                [
                    self.coeffs[1][0] + other.coeffs[1][0],
                    self.coeffs[1][1] + other.coeffs[1][1],
                    self.coeffs[1][2] + other.coeffs[1][2],
                ],
            ]
        }
    }
}

impl Sub for AugmentedMat2x2
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output
    {
        self + (other * -1.0)
    }
}

impl PartialEq for AugmentedMat2x2
{
    fn eq(&self, other: &Self) -> bool
    {
        let mut coeff_pairs = 
            utils::zip((&self.coeffs, &other.coeffs))
            .flat_map(utils::zip);

        coeff_pairs.all(|(c1, c2)| c1 == c2)
    }
}

#[cfg(test)]
mod tests
{
    use super::Vec2D;
    use super::AugmentedMat2x2;

    #[test]
    fn test_add_vec2d()
    {
        let x = Vec2D { x: 5., y: 10. };
        let y = Vec2D { x: 15., y: 10. };

        assert_eq!(x + y, Vec2D { x: 20., y: 20. });
    }

    #[test]
    fn test_sub_vec2d()
    {
        let x = Vec2D { x: 5., y: 10. };
        let y = Vec2D { x: 15., y: 10. };

        assert_eq!(x - y, Vec2D { x: -10., y: 0. });
    }

    #[test]
    fn test_augmented_mat_vec_mul()
    {
        let x = Vec2D { x: 5., y: 10. };
        let m = AugmentedMat2x2::new
        (
            [[1., 2., 3.],
             [4., 5., 6.]]
        );

        assert_eq!(m * x, Vec2D { x: 28., y: 76. });
    }

    #[test]
    fn test_augmented_mat_scalar_mul()
    {
        let x = Vec2D { x: 5., y: 10. };
        let m = AugmentedMat2x2::new
        (
            [[1., 2., 3.],
             [4., 5., 6.]]
        );

        assert_eq!(m * x, Vec2D { x: 28., y: 76. });
    }

    #[test]
    fn test_augmented_mat_add()
    {
        let m1 = AugmentedMat2x2::new
        (
            [[1., 2., 3.],
             [4., 5., 6.]]
        );
        let m2 = AugmentedMat2x2::new
        (
            [[5., 7., 9.],
             [1., 4., 0.]]
        );

        assert_eq!(m1 + m2, AugmentedMat2x2::new
        (
            [[6., 9., 12.],
             [5., 9., 6.]]
        ));

    }

    #[test]
    fn test_augmented_mat_mul_scalar()
    {
        let m1 = AugmentedMat2x2::new
        (
            [[1., 2., 3.],
             [4., 5., 6.]]
        );

        assert_eq!(-1. * m1, AugmentedMat2x2::new
        (
            [[-1., -2., -3.],
             [-4., -5., -6.]]
        ));

        assert_eq!(10. * m1, AugmentedMat2x2::new
        (
            [[10., 20., 30.],
             [40., 50., 60.]]
        ));
    }

    #[test]
    fn test_augmented_mat_sub()
    {
        let m1 = AugmentedMat2x2::new
        (
            [[1., 2., 3.],
             [4., 5., 6.]]
        );
        let m2 = AugmentedMat2x2::new
        (
            [[5., 7., 9.],
             [1., 4., 0.]]
        );

        assert_eq!(m1 - m2, AugmentedMat2x2::new
        (
            [[-4., -5., -6.],
             [3., 1., 6.]]
        ));
    }
}