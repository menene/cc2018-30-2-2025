## LLM Agent Instructions

When working with this project, please follow these instructions:

-   **Prioritize `raylib::prelude`:** Whenever possible, use the `raylib::prelude` module to import `raylib` components. This helps to keep the code clean and consistent.
-   **Avoid external libraries:** Do not introduce any new external libraries besides `raylib`. The goal is to keep the project simple and focused on the core concepts of raytracing.
-   **Prioritize code legibility:** The code in this project is intended to be used as a learning tool for students. Prioritize code that is easy to read and understand, even if it is not the most performant solution.
-   **Avoid excessive comments:** Do not add excessive comments to the code. The code should be self-documenting as much as possible. Only add comments to explain complex or non-obvious logic.
-   **Raylib Initialization:** Initialize `raylib` using the following pattern:
    ```rust
    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raytracer Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();
    ```
-   **Render Loop:** The main render loop should follow this structure:
    ```rust
    while !window.window_should_close() {
        framebuffer.clear();

        render(&mut framebuffer, &objects);

        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
    ```
-   **Verification:** After making any code changes, compile the project by running `cargo build`. If any errors occur, you must fix them before finishing your task.