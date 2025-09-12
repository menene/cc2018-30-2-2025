# Raytracer in Rust

This project is a simple raytracer written in Rust using the `raylib` library. It is intended as an educational tool for students learning about computer graphics.

## What is a Raycaster?

A raycaster is a rendering technique that creates a 3D perspective in a 2D map. The basic idea is to trace rays from the "eye" of the viewer through each pixel on the screen. The color of the pixel is then determined by what the ray hits in the scene.

This project implements a simple ray-sphere intersection algorithm to render a sphere on the screen.

## The Orbit Camera

This project uses an orbit camera to view the 3D scene. An orbit camera is a type of camera that rotates around a fixed central point, always keeping its focus on that point. This is different from a free-look camera, which can move and look in any direction independently. Orbit cameras are common in 3D modeling software and games where the user needs to inspect a central object from various angles.

### Core Concepts

The orbit camera implementation in `src/camera.rs` relies on several key concepts from linear algebra and trigonometry, which are fundamental to computer graphics.

#### Linear Algebra

*   **Vectors**: The camera's state is defined by 3D vectors:
    *   `eye`: The position of the camera in world space.
    *   `center`: The point the camera is looking at. This is the pivot point for the orbit.
    *   `up`: A vector that indicates the "up" direction for the camera, used to orient it correctly.

*   **Orthonormal Basis**: To properly orient the camera, we create a new coordinate system relative to the camera's view, called an orthonormal basis. This basis consists of three mutually perpendicular unit vectors:
    1.  `forward`: The direction the camera is looking. Calculated by normalizing the vector from the `eye` to the `center` (`center - eye`).
    2.  `right`: The direction to the right of the camera's view. Calculated using the **cross product** of the `forward` vector and the world's `up` vector.
    3.  `up` (camera's up): The "true" up direction for the camera. It's recalculated by taking the **cross product** of the `right` and `forward` vectors. This ensures the basis is perfectly orthogonal.

*   **Change of Basis**: When we cast rays from the camera, we define them in camera space and then transform them into world space. The `basis_change` function performs this transformation. It takes a vector defined in the camera's coordinate system (using `right`, `up`, `forward` as axes) and converts it to the world coordinate system, allowing us to correctly trace its path in the scene.

#### Trigonometry and Spherical Coordinates

*   **Rotation**: To make the camera "orbit", we need to rotate its `eye` position around the `center`. While this can be done with rotation matrices, a more intuitive way is to use spherical coordinates.
*   **Spherical Coordinates**: We can represent the camera's position by its `(radius, yaw, pitch)` relative to the `center`.
    *   `radius`: The distance from the `eye` to the `center`. This remains constant in an orbit camera.
    *   `yaw`: The horizontal angle (rotation around the vertical axis).
    *   `pitch`: The vertical angle (elevation from the horizontal plane).
*   **Conversion**: The `orbit` function works by:
    1.  Calculating the current `yaw` and `pitch` from the `eye`'s Cartesian coordinates.
    2.  Adding the user's input to these angles.
    3.  Converting the new `(radius, yaw, pitch)` back into Cartesian coordinates to get the new `eye` position.
    4.  The `pitch` is clamped to prevent the camera from flipping over and to avoid **gimbal lock**, a phenomenon that can cause loss of rotational control.

## The Phong Lighting Model

To give the objects in our scene a more realistic, 3D appearance, we use a lighting model. This project implements the Phong reflection model, a popular technique in computer graphics for calculating the color of a point on a surface. The calculation is done in the `cast_ray` function in `src/main.rs`.

The Phong model simulates how light interacts with a surface by breaking it down into three components:

1.  **Ambient**: Simulates indirect light that fills the scene. This component ensures that objects are never in complete darkness. (Note: This specific implementation omits the ambient component for simplicity).
2.  **Diffuse**: Simulates the light that hits a surface and scatters equally in all directions. This is what gives objects their base color. The brightness of the diffuse light depends on the angle between the surface normal and the light source.
3.  **Specular**: Simulates the bright, shiny highlights that appear on smooth surfaces. The intensity of the specular highlight depends on the viewing angle.

### Linear Algebra for Phong Lighting

Implementing the Phong model requires several key vector operations:

*   **Vector as Color**: In this implementation, we use `Vector3` to represent colors during the lighting calculations. This allows us to use standard vector operations like multiplication and addition directly on the colors. The `x`, `y`, and `z` components of the vector correspond to the R, G, and B channels of the color. At the end of the `cast_ray` function, the final `Vector3` is converted back to a `raylib::Color`.

*   **Normalization**: To ensure our calculations are based on directions rather than magnitudes, we use normalized (unit) vectors for:
    *   `light_dir`: The direction from the point on the surface to the light source.
    *   `view_dir`: The direction from the point on the surface to the camera.
    *   `normal`: The vector perpendicular to the surface at the point of intersection.
    *   `reflect_dir`: The direction of the reflected light.

*   **Dot Product**: The dot product is used to determine the intensity of the light components:
    *   **Diffuse Intensity**: Calculated by the dot product of the `light_dir` and the surface `normal`. The result is clamped to a minimum of 0, as a negative value would mean the light is behind the surface.
    *   **Specular Intensity**: Calculated by the dot product of the `view_dir` and the `reflect_dir`. A higher dot product means the camera is more aligned with the reflection direction, resulting in a brighter highlight.

*   **Vector Reflection**: To find the direction of the specular highlight, we need to calculate the reflection of the light vector. The `reflect` function calculates this using the formula: `R = L - 2 * N * dot(L, N)`, where `L` is the incident light vector and `N` is the surface normal.

These components are calculated and then combined to produce the final color of the pixel, giving the rendered objects a simple but effective illusion of depth and material.

## Shadows

To add another layer of realism, this ray tracer implements hard shadows. A point on a surface is in shadow if it is blocked from the light source by another object.

### Shadow Rays

The core idea is simple: from the point of intersection on a surface, we cast a second ray, called a **shadow ray**, towards the light source.

*   **Linear Algebra for Shadows**: The primary operation is another **ray-object intersection test**. We create a new ray starting from the surface point and pointing in the direction of the light. We then check if this ray hits any other object in the scene.

*   **Condition for Shadow**: If the shadow ray intersects an object before it reaches the light source, the point is in shadow. If the ray reaches the light source without any intersections, the point is lit.

### Shadow Acne and Bias

A common problem when implementing shadows is a graphical artifact called **shadow acne**. This happens when a shadow ray accidentally intersects the very same surface it originated from, due to floating-point precision errors. This causes the surface to incorrectly shadow itself, creating a speckled or "acne-like" pattern.

To solve this, we use a **shadow bias**. We slightly offset the starting point of the shadow ray along the surface normal. This pushes the ray's origin just enough to avoid an immediate self-intersection.

*   In `cast_shadow`, we calculate an `offset_normal` by multiplying the surface normal by a small `SHADOW_BIAS` value.
*   This offset is then added to the intersection point, giving us a safe starting position for the shadow ray.

The final light intensity is then attenuated based on whether the point is in shadow, and this adjusted intensity is used in the Phong lighting calculation.

## Reflections

To create surfaces that look like mirrors, we need to implement reflections. This is where ray tracing truly becomes recursive.

### Recursive Ray Tracing

When a ray hits a reflective surface, we need to figure out what color it "sees" in the reflection. We do this by casting a new ray from the intersection point in the direction of the reflection. The color returned by this new ray is then blended with the object's own color.

*   **Linear Algebra for Reflection**: The key is to calculate the reflection direction. This is done with the same `reflect` function we used for specular highlights. This time, instead of reflecting the light direction, we reflect the *viewing direction* (the ray coming from the camera). The formula `R = V - 2 * N * dot(V, N)` gives us the new direction for our reflection ray, where `V` is the incoming view vector and `N` is the surface normal.

*   **Blending Colors**: The `albedo` property of our `Material` now has a third component that controls the reflectivity of the surface. The final color is a blend of the object's Phong color and the color returned by the reflection ray, weighted by this reflectivity value.

### Recursion Depth

Casting a ray that hits a mirror, which reflects another mirror, could lead to an infinite loop of reflections. To prevent this, we add a `depth` parameter to our `cast_ray` function. Each time we cast a reflection ray, we increment the depth. If the depth exceeds a certain limit, we stop recursing and return a default background color. This ensures the program doesn't get stuck in an infinite loop.

## Refractions

Refraction is the bending of light as it passes from one medium to another, like from air to water or glass. This is what makes objects appear distorted when viewed through a transparent material.

### Snell's Law

The direction of the refracted ray is calculated using Snell's Law. For students who might have zoned out during this part of physics or linear algebra, here's the gist:

*   **Refractive Index**: Every transparent material has a `refractive_index` that describes how much it slows down light. Air is typically ~1.0, water is ~1.33, and glass is ~1.5.
*   **The Bend**: Snell's Law uses the incoming ray's direction, the surface normal, and the ratio of the two materials' refractive indices to calculate the new, bent direction of the ray. Our `refract` function implements this calculation.

### Total Internal Reflection

Sometimes, when light tries to exit a dense medium (like glass) into a less dense one (like air) at a very shallow angle, it can't escape and reflects back inside instead. This is called **Total Internal Reflection**.

Our `refract` function automatically handles this. If the calculation for the refracted ray has no real solution (which happens when the light hits at that shallow angle), the function instead returns the *reflection* direction. This is why our glass sphere might look like a perfect mirror from certain angles.

### Blending and Recursion

Just like with reflections, we cast a new ray in the refracted direction and recursively call `cast_ray`. The `albedo` property of our `Material` now has a fourth component that controls the transparency of the surface. The final color is a blend of the Phong color, the reflected color, and the refracted color, all weighted by their respective albedo values.

## How to run this code

To run this code, you will need to have Rust installed. You can find instructions on how to install Rust [here](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can clone this repository and run the following command in the root directory of the project:

```bash
cargo run --release
```

This will compile and run the project. A window should appear with a rendered sphere.

## File Structure

The project is organized into the following files:

-   `src/main.rs`: The main entry point of the program. It initializes `raylib`, creates a window, and contains the main render loop and lighting calculations.
-   `src/camera.rs`: Implements the orbit camera, including its orientation and movement logic.
-   `src/framebuffer.rs`: This file contains the `Framebuffer` struct, which is used to store the rendered image before it is displayed on the screen.
-   `src/light.rs`: Defines the `Light` struct, representing a light source in the scene.
-   `src/material.rs`: Defines the `Material` struct and the `vector3_to_color` conversion function.
-   `src/ray_intersect.rs`: This file defines the `RayIntersect` trait, which is used to check if a ray intersects with an object in the scene.
-   `src/sphere.rs`: This file contains the `Sphere` struct and its implementation of the `RayIntersect` trait.