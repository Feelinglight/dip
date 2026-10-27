use std::{
    error::Error,
    fmt,
    ops::{Deref, DerefMut},
};

use image::{ImageBuffer, Pixel, Primitive};
use num_traits::{self, ToPrimitive};

use super::types::Dot;

pub const PIECEWISE_DOTS_MAX: usize = 255;

#[derive(Debug)]
pub enum PiecewiceLinearError {
    TooFewDotsSpecified,
    FirstDotIsNotZero(Dot),
    LastDotIsNotOne(Dot),
    NotIncreasing(Vec<(usize, Dot, Dot)>),
    ValueOutOfRange(Vec<(usize, Dot)>),
}

impl Error for PiecewiceLinearError {}

impl fmt::Display for PiecewiceLinearError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PiecewiceLinearError::TooFewDotsSpecified => {
                write!(
                    f,
                    "должно быть задано как минимум 2 точки кусочно-линейного преобразования"
                )?;
            }
            PiecewiceLinearError::NotIncreasing(dots) => {
                write!(
                    f,
                    "некорректные параметры кусочно-линейного преобразования:"
                )?;
                for (idx, dot, dot_prev) in dots {
                    write!(
                        f,
                        "точка {idx} {dot}: координата X меньше координаты X предыдущей \
                        точки {dot_prev}",
                    )?;
                }
            }
            PiecewiceLinearError::ValueOutOfRange(dots) => {
                write!(
                    f,
                    "некорректные параметры кусочно-линейного преобразования:"
                )?;
                for (idx, dot) in dots {
                    write!(
                        f,
                        "координаты точки {idx} {dot} должны быть в диапазоне от 0 до 1",
                    )?;
                }
            }
            PiecewiceLinearError::FirstDotIsNotZero(dot) => {
                write!(
                    f,
                    "первая точка кусочно-линейного преобразования {dot} имеет ненулевую \
                    координату по X"
                )?;
            }
            PiecewiceLinearError::LastDotIsNotOne(dot) => {
                write!(
                    f,
                    "последняя точка кусочно-линейного преобразования {dot} должна иметь значение \
                    1 по X"
                )?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PiecewiseLinearParams {
    dots: Vec<Dot>,
    coefs: Vec<(f64, f64, f64, f64)>,
}

impl Default for PiecewiseLinearParams {
    fn default() -> Self {
        Self::try_new(vec![Dot { x: 0., y: 0. }, Dot { x: 1., y: 0. }])
            .expect("дефолтные значения параметров всегда должны быть валидны")
    }
}

fn check_monotonicity(dots: &[Dot]) -> Result<(), PiecewiceLinearError> {
    let mut not_increasing_dots = vec![];

    for ((idx, dot), next_dot) in dots.iter().enumerate().zip(dots.iter().skip(1)) {
        if next_dot.x <= dot.x {
            not_increasing_dots.push((idx + 2, *next_dot, *dot));
        }
    }

    if not_increasing_dots.is_empty() {
        Ok(())
    } else {
        Err(PiecewiceLinearError::NotIncreasing(not_increasing_dots))
    }
}

fn check_range(dots: &[Dot]) -> Result<(), PiecewiceLinearError> {
    let mut out_of_range_dots = vec![];

    for (idx, dot) in dots.iter().enumerate() {
        if !(0.0..=1.).contains(&dot.x) || !(0.0..=1.).contains(&dot.y) {
            out_of_range_dots.push((idx + 1, *dot));
        }
    }

    if out_of_range_dots.is_empty() {
        Ok(())
    } else {
        Err(PiecewiceLinearError::ValueOutOfRange(out_of_range_dots))
    }
}

impl PiecewiseLinearParams {
    /// # Errors
    ///
    /// Возвращает [`PiecewiceLinearError`], если заданные точки не валидны
    pub fn try_new(dots: Vec<Dot>) -> Result<Self, PiecewiceLinearError> {
        let [first, .., last] = dots.as_slice() else {
            return Err(PiecewiceLinearError::TooFewDotsSpecified);
        };
        if first.x != 0. {
            return Err(PiecewiceLinearError::FirstDotIsNotZero(*first));
        }
        if last.x != 1. {
            return Err(PiecewiceLinearError::LastDotIsNotOne(*last));
        }

        check_monotonicity(&dots)?;
        check_range(&dots)?;

        let mut coefs = Vec::with_capacity(dots.len() - 1);
        for (Dot { x: x1, y: y1 }, Dot { x: x2, y: y2 }) in dots.iter().zip(dots.iter().skip(1)) {
            coefs.push((*x1, *y1, x2 - x1, y2 - y1));
        }

        Ok(Self { dots, coefs })
    }

    #[must_use]
    pub const fn dots(&self) -> &[Dot] {
        self.dots.as_slice()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[inline]
#[must_use]
pub fn piecewise_linear_single(value: f64, params: &PiecewiseLinearParams) -> f64 {
    let dots = params.dots();
    let value = value.clamp(0., 1.);

    // Если value меньше либо равно координате X первой точки, то берем значение из первой точки
    // Если value больше координаты X последней точки, то берем значение из последней точки
    let interval_idx = dots
        .iter()
        .position(|dot| value <= dot.x)
        .unwrap_or(dots.len() - 1)
        .saturating_sub(1);

    #[expect(clippy::missing_panics_doc)]
    let (x1, y1, dx, dy) = params
        .coefs
        .get(interval_idx)
        .expect("номер интервала превышен");

    y1 + (value - x1) / dx * dy
}

pub trait PiecewiseLinear: Sized {
    fn piecewise_linear_inplace(&mut self, params: &PiecewiseLinearParams);

    #[must_use]
    fn piecewise_linear(mut self, params: &PiecewiseLinearParams) -> Self {
        self.piecewise_linear_inplace(params);
        self
    }
}

impl<P, Container> PiecewiseLinear for ImageBuffer<P, Container>
where
    P: Pixel,
    P::Subpixel: num_traits::PrimInt,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
    fn piecewise_linear_inplace(&mut self, params: &PiecewiseLinearParams) {
        let max_value = P::Subpixel::DEFAULT_MAX_VALUE.to_f64().unwrap_or(1.);

        for sample in self.iter_mut() {
            let fsample = (*sample).to_f64().unwrap_or(0.);
            let norm_fsample = fsample / max_value;

            let corrected = (piecewise_linear_single(norm_fsample, params) * max_value).round();
            *sample =
                num_traits::NumCast::from(corrected).unwrap_or(P::Subpixel::DEFAULT_MIN_VALUE);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    use rstest::rstest;
    use std::assert_matches;

    #[rstest]
    fn test_default_doesnt_panic() {
        PiecewiseLinearParams::default();
    }

    mod params_try_new {
        use super::*;

        #[rstest]
        fn test_dots_ok() {
            let dots = vec![Dot { x: 0., y: 0. }, Dot { x: 1., y: 0. }];
            assert_matches!(PiecewiseLinearParams::try_new(dots), Ok(_));
        }

        #[rstest]
        #[case(vec![])]
        #[case(vec![Dot {x: 0., y: 0.}])]
        fn test_try_new_too_few_dots(#[case] dots: Vec<Dot>) {
            assert_matches!(
                PiecewiseLinearParams::try_new(dots),
                Err(PiecewiceLinearError::TooFewDotsSpecified)
            );
        }

        #[rstest]
        fn first_x_is_not_zero() {
            let first_dot = Dot { x: 1., y: 0. };
            let dots = vec![first_dot, Dot { x: 1., y: 0. }];
            assert_matches!(
                PiecewiseLinearParams::try_new(dots),
                Err(PiecewiceLinearError::FirstDotIsNotZero(dot)) if dot == first_dot
            );
        }

        #[rstest]
        fn last_x_is_not_one() {
            let last_dot = Dot { x: 0.5, y: 0. };
            let dots = vec![Dot { x: 0., y: 0.5 }, last_dot];
            assert_matches!(
                PiecewiseLinearParams::try_new(dots),
                Err(PiecewiceLinearError::LastDotIsNotOne(dot)) if dot == last_dot
            );
        }

        #[rstest]
        fn first_x_is_not_zero_and_last_x_is_not_one() {
            let first_dot = Dot { x: 1., y: 0. };
            let dots = vec![first_dot, Dot { x: 0.5, y: 0.5 }];
            assert_matches!(
                PiecewiseLinearParams::try_new(dots),
                Err(PiecewiceLinearError::FirstDotIsNotZero(dot)) if dot == first_dot
            );
        }

        #[rstest]
        #[case::first_dot_the_same(
            vec![Dot {x: 0., y: 1.}, Dot {x: 0., y: 2.}, Dot {x: 1., y: 3.}], vec![1])
        ]
        #[case::first_dot_the_same(
            vec![
                Dot {x: 0., y: 1.}, Dot {x: 0.5, y: 2.}, Dot {x: 0.5, y: 3.}, Dot {x: 1., y: 4.}],
                vec![2]
            )
        ]
        #[case::last_dot_the_same(
            vec![Dot {x: 0., y: 1.}, Dot {x: 1., y: 2.}, Dot {x: 1., y: 3.}], vec![2])
        ]
        #[case::multiple_dots_the_same(
                vec![
                    Dot {x: 0., y: 1.},
                    Dot {x: 0., y: 2.},
                    Dot {x: 0., y: 3.},
                    Dot {x: 0.5, y: 4.},
                    Dot {x: 0.4, y: 5.},
                    Dot {x: 0.3, y: 6.},
                    Dot {x: 0.2, y: 7.},
                    Dot {x: 1., y: 8.},
                    Dot {x: 1., y: 9.}
                ],
                vec![1, 2, 4, 5, 6, 8]
            )
        ]
        fn test_dots_not_increasing(#[case] dots: Vec<Dot>, #[case] error_indexes: Vec<usize>) {
            #[allow(clippy::indexing_slicing)]
            let expected_dots = error_indexes
                .iter()
                .map(|idx| (*idx + 1, dots[*idx], dots[*idx - 1]))
                .collect::<Vec<_>>();

            let Err(PiecewiceLinearError::NotIncreasing(bad_dots)) =
                PiecewiseLinearParams::try_new(dots)
            else {
                unreachable!();
            };

            assert_eq!(bad_dots, expected_dots);
        }
    }

    mod apply_single {
        use super::*;

        #[rstest]
        fn test_simple_linear_transform() {
            let Ok(params) =
                PiecewiseLinearParams::try_new(vec![Dot { x: 0., y: 0. }, Dot { x: 1., y: 1. }])
            else {
                unreachable!();
            };

            let mut results = Vec::with_capacity(10);
            let mut expected = Vec::with_capacity(10);

            let mut value = 0.;
            for _ in 0..=10 {
                expected.push(value);
                results.push(piecewise_linear_single(value, &params));
                value += 0.1;
            }

            assert_eq!(results, expected);
        }
    }

    #[rstest]
    fn test_complex_linear_transform() {
        let Ok(params) = PiecewiseLinearParams::try_new(vec![
            Dot { x: 0., y: 0. },
            Dot { x: 0.2, y: 0. },
            Dot { x: 0.4, y: 0.4 },
            Dot { x: 0.6, y: 0.4 },
            Dot { x: 0.8, y: 0. },
            Dot { x: 1., y: 0. },
        ]) else {
            unreachable!();
        };

        let mut results = Vec::with_capacity(11);

        let mut value = 0.;
        for _ in 0..11 {
            results.push(piecewise_linear_single(value, &params));
            value += 0.1;
        }

        let expected = vec![0., 0., 0., 0.2, 0.4, 0.4, 0.4, 0.2, 0., 0., 0.];

        assert!(
            results
                .iter()
                .zip(expected)
                .all(|(res, exp)| utils::approx_eq(*res, exp))
        );
    }

    #[rstest]
    #[case::out_of_range_right(-1.)]
    #[case::out_of_range_left(2.)]
    fn test_simple_linear_transform(#[case] value: f64) {
        let start_transform_value = 0.3;
        let end_transform_value = 0.7;
        let Ok(params) = PiecewiseLinearParams::try_new(vec![
            Dot {
                x: 0.,
                y: start_transform_value,
            },
            Dot {
                x: 1.,
                y: end_transform_value,
            },
        ]) else {
            unreachable!();
        };

        let result = piecewise_linear_single(value, &params);
        let expected = if value < 0. {
            start_transform_value
        } else {
            end_transform_value
        };

        assert_eq!(result, expected);
    }

    #[rstest]
    fn test_slope_overflow() {
        let Ok(params) = PiecewiseLinearParams::try_new(vec![
            Dot { x: 0., y: 0. },
            Dot { x: 1e-320, y: 0.5 },
            Dot { x: 1., y: 0.5 },
        ]) else {
            unreachable!();
        };

        let result = piecewise_linear_single(0., &params);

        assert_eq!(result, 0.);
    }

    #[rstest]
    fn test_short_interval() {
        let Ok(params) = PiecewiseLinearParams::try_new(vec![
            Dot { x: 0., y: 0. },
            Dot {
                x: 0.999_999_999_999_999_9,
                y: 0.1,
            },
            Dot { x: 1., y: 0.9 },
        ]) else {
            unreachable!();
        };

        let result = piecewise_linear_single(1., &params);
        assert_eq!(result, 0.9);
    }
}
