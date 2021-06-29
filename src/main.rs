fn main() {
    let gl_versions = [
        (3, 1),
        (3, 2),
        (3, 3),
        (4, 0),
        (4, 1),
        (4, 2),
        (4, 3),
        (4, 4),
        (4, 5),
        (4, 6),
    ];

    for &version in gl_versions.iter() {
        let gl_request = glutin::GlRequest::Specific(glutin::Api::OpenGl, version);
        let event_loop = glutin::event_loop::EventLoop::new();
        let size = glutin::dpi::PhysicalSize::new(1000, 1000);

        let result = glutin::ContextBuilder::new()
            .with_gl(gl_request)
            .with_gl_profile(glutin::GlProfile::Core)
            .build_headless(&event_loop, size);

        match result {
            Err(error) => {
                println!("Version {:?} is not supported: {:?}", version, error);
            }
            Ok(context) => unsafe {
                if context.make_current().is_ok() {
                    println!("Version {:?} is supported", version);
                } else {
                    println!(
                        "Version {:?} is not supported: couldn't make context current",
                        version
                    );
                }
            },
        }
    }
}
