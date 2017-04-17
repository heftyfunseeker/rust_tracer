
static CHANNELS_PER_PIXEL:usize = 3;

pub struct RenderBufferI32 {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<i32>,
}

impl RenderBufferI32 {
    pub fn new(width: usize, height: usize) -> RenderBufferI32  {
        let size = width * height * CHANNELS_PER_PIXEL;
        return RenderBufferI32 {
            width: width,
            height: height,
            buffer: Vec::with_capacity(size)
        }
    }

    pub fn push_pixel(
        &mut self,
        r: i32,
        g: i32,
        b: i32
    ) {
        self.buffer.push(r);
        self.buffer.push(g);
        self.buffer.push(b);
    }
}
