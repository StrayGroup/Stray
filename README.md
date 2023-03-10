<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://user-images.githubusercontent.com/78795073/213866505-dbe1bb08-105a-40ae-b403-2b914cf8a740.png">
  <source media="(prefers-color-scheme: light)" srcset="https://user-images.githubusercontent.com/78795073/213866506-2fc3b40e-1d34-42ce-8e97-eacaf46632ce.png">
  <img alt="Stray">
</picture>

# An Experimental 2D Game Engine

***In Early Stage Of Development***

## Features:
- **2D Renderer in WGPU**
- **API which uses Legion ECS**
- **Drawing System**
- **Texture Rendering**
- **Input System**
- **SMAA Support (Commented code, maybe works on better hardware than I have)**

## Nearby Planned Features:
- **Camera System**
- **Text Rendering**
- **Full Plugin System**


## Getting Started
To draw a window, use:
```rust
use stray::prelude::*;

fn main(){
  Stray::new()
    .with_title("Hello World")
    .build()
    .run();
}
```

## Dependences Used:
- **[WGPU](https://github.com/gfx-rs/wgpu)**
- **[Legion ECS](https://github.com/amethyst/legion)**
- **[Glam](https://github.com/bitshifter/glam-rs)**

## Contributing
Any contribution is welcome, no matter what your skills are, create issues and make pull requests that close them or other issues
