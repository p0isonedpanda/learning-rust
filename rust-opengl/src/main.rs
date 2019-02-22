extern crate sdl2;
extern crate gl;

fn main()
{
    let _sdl = sdl2::init().unwrap();
    let video_subsystem = _sdl.video().unwrap();
    let window = video_subsystem
        .window("main", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe
    {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut event_pump = _sdl.event_pump().unwrap();
    'main: loop
    {
        for event in event_pump.poll_iter()
        {
            match event
            {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        unsafe
        {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }
}
