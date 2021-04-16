(function() {var implementors = {};
implementors["ray_tracing"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"ray_tracing/camera/struct.Camera.html\" title=\"struct ray_tracing::camera::Camera\">Camera</a>","synthetic":true,"types":["ray_tracing::camera::Camera"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"enum\" href=\"ray_tracing/error/enum.ErrorKind.html\" title=\"enum ray_tracing::error::ErrorKind\">ErrorKind</a>","synthetic":true,"types":["ray_tracing::error::ErrorKind"]},{"text":"impl&lt;'world&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"ray_tracing/hittable/struct.HitRecord.html\" title=\"struct ray_tracing::hittable::HitRecord\">HitRecord</a>&lt;'world&gt;","synthetic":true,"types":["ray_tracing::hittable::HitRecord"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"ray_tracing/hittable/struct.HittableList.html\" title=\"struct ray_tracing::hittable::HittableList\">HittableList</a>","synthetic":true,"types":["ray_tracing::hittable::HittableList"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"ray_tracing/ray/struct.Ray.html\" title=\"struct ray_tracing::ray::Ray\">Ray</a>","synthetic":true,"types":["ray_tracing::ray::Ray"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"ray_tracing/sphere/struct.Sphere.html\" title=\"struct ray_tracing::sphere::Sphere\">Sphere</a>","synthetic":true,"types":["ray_tracing::sphere::Sphere"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"ray_tracing/vec/raw/struct.Vec2d.html\" title=\"struct ray_tracing::vec::raw::Vec2d\">Vec2d</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>","synthetic":true,"types":["ray_tracing::vec::raw::Vec2d"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"ray_tracing/vec/raw/struct.Vec3d.html\" title=\"struct ray_tracing::vec::raw::Vec3d\">Vec3d</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>","synthetic":true,"types":["ray_tracing::vec::raw::Vec3d"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"ray_tracing/vec/raw/struct.Vec4d.html\" title=\"struct ray_tracing::vec::raw::Vec4d\">Vec4d</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>","synthetic":true,"types":["ray_tracing::vec::raw::Vec4d"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"ray_tracing/vec/struct.Vec3d.html\" title=\"struct ray_tracing::vec::Vec3d\">Vec3d</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>","synthetic":true,"types":["ray_tracing::vec::Vec3d"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"ray_tracing/vec/struct.Color.html\" title=\"struct ray_tracing::vec::Color\">Color</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>","synthetic":true,"types":["ray_tracing::vec::Color"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"ray_tracing/material/struct.Lambertian.html\" title=\"struct ray_tracing::material::Lambertian\">Lambertian</a>","synthetic":true,"types":["ray_tracing::material::Lambertian"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"ray_tracing/material/struct.Metal.html\" title=\"struct ray_tracing::material::Metal\">Metal</a>","synthetic":true,"types":["ray_tracing::material::Metal"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()