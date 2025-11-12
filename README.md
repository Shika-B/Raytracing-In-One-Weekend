A single-day quick implementation of the [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) book. 
The goal was to get something reasonably clean working fast, and I believe I achieved that.

Features checklist:
[X] PPM image creation
[X] Ray-sphere intersection, normals (and all the basic linear algebra and geometry. Vector boilerplates is mostly the nalgebra crate) 
[X] Boxed anti-aliasing
[X] Clean movable vector camera (TODO: Clean everything and store proj matrices instead)
[X] Naive and Lambertian diffuse material
[X] Metallic material
[ ] Dieletrics
[ ] Defocus blur