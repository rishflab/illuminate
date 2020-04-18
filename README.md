# Illuminate
Illuminate is a game engine that features a real-time pathtracing renderer and an entity-component-system
architecture for cache-optimised multithreaded game logic.

## Hardware Support
Currently the GPU pathtracer requires a Vulkan capable GPU but Metal and DX12 will be supported in the future.

The pathtracer will not work if the GPU does not support STORAGE usage for swapchain images (see issue #7).

A offline CPU pathtracer is being simultaneously developed for fast prototyping of ideas before
they are implemented on the GPU for real-time performance.

### Confirmed Supported Hardware
- AMD Radeon RX 570
- AMD Radeon RX 580

