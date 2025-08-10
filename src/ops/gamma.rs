use crate::color_conversions::*;
use crate::opbasics::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct OpGamma {}

impl OpGamma {
    pub fn new(_img: &ImageSource) -> OpGamma {
        OpGamma {}
    }
}

impl<'a> ImageOp<'a> for OpGamma {
    fn name(&self) -> &str {
        "gamma"
    }

    fn run(&self, pipeline: &PipelineGlobals, buf: Arc<OpBuffer>) -> Arc<OpBuffer> {
        if pipeline.settings.linear {
            buf
        } else {
            Arc::new(buf.mutate_lines_copying(
                &(|line: &mut [f32], _| {
                    for pix in line.chunks_exact_mut(1) {
                        pix[0] = apply_srgb_gamma(pix[0].clamp(0.0, 1.0));
                    }
                }),
            ))
        }
    }
}
