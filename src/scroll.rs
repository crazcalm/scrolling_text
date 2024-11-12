use crate::Frame;
use core::clone::Clone;
use core::fmt::Debug;
use core::iter::Iterator;
use core::marker::Copy;
use core::option::Option::{self, None, Some};

use core::prelude::rust_2024::derive;

#[derive(Debug, Copy, Clone)]
pub struct ScrollFrame {
    letter: Frame,
    current_column: usize,
    current_frame: [[u8; 5]; 5],
    end_buffer_frames: u8,
}

impl ScrollFrame {
    pub fn new(letter: Frame) -> Self {
        let current_frame = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];

        Self {
            letter,
            current_frame,
            current_column: 0,
            end_buffer_frames: 5,
        }
    }
}

impl Iterator for ScrollFrame {
    type Item = [[u8; 5]; 5];

    fn next(&mut self) -> Option<Self::Item> {
        fn shift_current_frame_left(frame: &mut [[u8; 5]; 5]) {
            for col in 1..5 {
                for row in 0..5 {
                    frame[row][col - 1] = frame[row][col]
                }
            }
        }

        let mut result = None;

        if self.current_column < 5 {
            shift_current_frame_left(&mut self.current_frame);

            //fill in last column
            for row in 0..5 {
                self.current_frame[row][4] = self.letter[row][self.current_column]
            }

            self.current_column += 1;

            result = Some(self.current_frame.clone());
        } else if self.end_buffer_frames > 0 {
            self.end_buffer_frames -= 1;
            shift_current_frame_left(&mut self.current_frame);

            // Fill in last column with zeros
            for row in 0..5 {
                self.current_frame[row][4] = 0;
            }

            result = Some(self.current_frame.clone());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::ScrollFrame;
    use crate::A;

    #[test]
    fn test_scrollframe_iterator() {
        let mut scrollframe = ScrollFrame::new(A);

        let cases = [
            [
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 1],
                [0, 0, 0, 0, 1],
                [0, 0, 0, 0, 1],
                [0, 0, 0, 0, 1],
            ],
            [
                [0, 0, 0, 0, 1],
                [0, 0, 0, 1, 0],
                [0, 0, 0, 1, 1],
                [0, 0, 0, 1, 0],
                [0, 0, 0, 1, 0],
            ],
            [
                [0, 0, 0, 1, 1],
                [0, 0, 1, 0, 0],
                [0, 0, 1, 1, 1],
                [0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0],
            ],
            [
                [0, 0, 1, 1, 1],
                [0, 1, 0, 0, 0],
                [0, 1, 1, 1, 1],
                [0, 1, 0, 0, 0],
                [0, 1, 0, 0, 0],
            ],
            [
                [0, 1, 1, 1, 0],
                [1, 0, 0, 0, 1],
                [1, 1, 1, 1, 1],
                [1, 0, 0, 0, 1],
                [1, 0, 0, 0, 1],
            ],
            [
                [1, 1, 1, 0, 0],
                [0, 0, 0, 1, 0],
                [1, 1, 1, 1, 0],
                [0, 0, 0, 1, 0],
                [0, 0, 0, 1, 0],
            ],
            [
                [1, 1, 0, 0, 0],
                [0, 0, 1, 0, 0],
                [1, 1, 1, 0, 0],
                [0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0],
            ],
            [
                [1, 0, 0, 0, 0],
                [0, 1, 0, 0, 0],
                [1, 1, 0, 0, 0],
                [0, 1, 0, 0, 0],
                [0, 1, 0, 0, 0],
            ],
            [
                [0, 0, 0, 0, 0],
                [1, 0, 0, 0, 0],
                [1, 0, 0, 0, 0],
                [1, 0, 0, 0, 0],
                [1, 0, 0, 0, 0],
            ],
            [
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
        ];

        for (index, frame) in scrollframe.enumerate() {
            assert_eq!(frame, cases[index], "Frame {index} is not equal");
        }
    }
}
