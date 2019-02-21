extern crate sdl2;

fn main()
{
    let _sdl = sdl2::init().unwrap();
    let video_subsystem = _sdl.video().unwrap();
    let window = video_subsystem
        .window("main", 900, 700)
        .resizable()
        .build()
        .unwrap();

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

        // render window contents here
    }
}
