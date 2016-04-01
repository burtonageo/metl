use StrongPtr;

pub struct Window(StrongPtr);

impl Window {
    pub fn size(&self) -> (usize, usize) {
        unimplemented!();
    }
}

pub enum WindowBorderStyle {
    Windowed,
    WindowedBorderless,
    FullScreen
}

pub enum WindowResizeControls {
    None,
    ResizeOnly,
    ResizeAndClose,
    All
}

pub enum ColorDepth {
    Depth24Bit,
    Depth32Bit
}

pub struct WindowBuilder {
    pub size: (usize, usize),
    pub border_style: WindowBorderStyle,
    pub controls: WindowResizeControls,
    pub color_depth: ColorDepth
}

impl WindowBuilder {
    pub fn window_size(self, size: (usize, usize)) -> Self {
        WindowBuilder { size: size, ..self }
    }

    pub fn border_style(self, border_style: WindowBorderStyle) -> Self {
        WindowBuilder { border_style: border_style, ..self }
    }

    pub fn controls(self, controls: WindowResizeControls) -> Self {
        WindowBuilder { controls: controls, ..self }
    }

    pub fn color_depth(self, color_depth: ColorDepth) -> Self {
        WindowBuilder { color_depth: color_depth, ..self }
    }

    pub fn build(self) -> Result<Window, ()> {
        unimplemented!();
    }
}
