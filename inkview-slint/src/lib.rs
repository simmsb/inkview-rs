use inkview::event::Key;
use inkview::{screen::Screen, Event};
use rgb::RGB;
use slint::platform::{
    software_renderer::{self as renderer, PhysicalRegion},
    WindowEvent,
};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::mpsc::Receiver,
    time::{Duration, Instant},
};

const SCALE_FACTOR: f32 = 3.0;

#[repr(transparent)]
#[derive(Clone, Copy)]
struct GrayScalePixel(u8);

impl GrayScalePixel {
    fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self(((red as u16 + green as u16 + blue as u16) / 3) as u8)
    }
}

pub struct Backend {
    screen: RefCell<Screen<'static>>,
    evts: Receiver<Event>,
    width: usize,
    height: usize,
    window: RefCell<Option<Rc<renderer::MinimalSoftwareWindow>>>,
    buffer: RefCell<Vec<RGB<u8>>>,
}

impl Backend {
    pub fn new(screen: Screen<'static>, evts: Receiver<Event>) -> Self {
        let width = screen.width();
        let height = screen.height();

        let buffer = vec![Default::default(); width * height];

        Self {
            screen: screen.into(),
            evts,
            width,
            height,
            window: Default::default(),
            buffer: buffer.into(),
        }
    }
}

fn rect_from_phys(r: PhysicalRegion) -> euclid::Rect<i32, euclid::UnknownUnit> {
    euclid::Rect::new(
        euclid::Point2D::new(r.bounding_box_origin().x, r.bounding_box_origin().y),
        euclid::Size2D::new(
            r.bounding_box_size().width as i32,
            r.bounding_box_size().height as i32,
        ),
    )
}

impl slint::platform::Platform for Backend {
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        let window =
            renderer::MinimalSoftwareWindow::new(renderer::RepaintBufferType::ReusedBuffer);
        self.window.replace(Some(window.clone()));
        Ok(window)
    }

    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        // self.window
        //     .borrow()
        //     .as_ref()
        //     .unwrap()
        //     .set_size(slint::PhysicalSize::new(
        //         self.width as u32,
        //         self.height as u32,
        //     ));

        slint::Window::set_size(
            self.window.borrow().as_ref().unwrap().as_ref(),
            slint::PhysicalSize::new(self.width as u32, self.height as u32)
                .to_logical(SCALE_FACTOR),
        );

        self.window
            .borrow()
            .as_ref()
            .unwrap()
            .dispatch_event(WindowEvent::ScaleFactorChanged {
                scale_factor: SCALE_FACTOR,
            });

        // slint::Window::set_size(
        //     self.window.borrow().as_ref().unwrap().as_ref(),
        //     slint::PhysicalSize::new(self.width as u32, self.height as u32).to_logical(SCALE_FACTOR),
        // );

        // bad naming, oops
        let mut fulfill_dynamic_updates_after: Option<Instant> = None;
        let mut dynamic_region_to_redraw: Option<euclid::Rect<i32, euclid::UnknownUnit>> = None;
        let mut accumulated_updates: Option<euclid::Rect<i32, euclid::UnknownUnit>> = None;
        let mut last_draw_at = Instant::now();

        loop {
            slint::platform::update_timers_and_animations();

            if let Some(window) = self.window.borrow().clone() {
                let delay = if window.has_active_animations() {
                    None
                } else {
                    match (
                        slint::platform::duration_until_next_timer_update(),
                        fulfill_dynamic_updates_after.map(|i| i.duration_since(Instant::now())),
                    ) {
                        (Some(a), Some(b)) => Some(a.min(b)),
                        (Some(a), None) => Some(a),
                        (_, b) => b,
                    }
                };

                let evt = if let Some(delay) = delay {
                    self.evts
                        .recv_timeout(delay)
                        .ok()
                        .and_then(ink_evt_to_slint)
                } else if window.has_active_animations() {
                    self.evts.try_recv().ok().and_then(ink_evt_to_slint)
                } else {
                    self.evts.recv().ok().and_then(ink_evt_to_slint)
                };

                if let Some(redraw_region) = dynamic_region_to_redraw {
                    if last_draw_at.elapsed() > Duration::from_millis(200) {
                        dynamic_region_to_redraw = None;
                        fulfill_dynamic_updates_after = None;

                        let mut screen = self.screen.borrow_mut();
                        screen.partial_update(
                            redraw_region.origin.x,
                            redraw_region.origin.y,
                            redraw_region.width() as u32,
                            redraw_region.height() as u32,
                        );
                        last_draw_at = Instant::now();
                    }
                }

                slint::platform::update_timers_and_animations();

                if let Some(evt) = evt {
                    window.dispatch_event(evt);
                }

                window.draw_if_needed(|renderer| {
                    let mut buffer = self.buffer.borrow_mut();
                    let damage = renderer.render(buffer.as_mut_slice(), self.width);
                    let mut screen = self.screen.borrow_mut();

                    for dy in 0..damage.bounding_box_size().height {
                        for dx in 0..damage.bounding_box_size().width {
                            let x = damage.bounding_box_origin().x + dx as i32;
                            let y = damage.bounding_box_origin().y + dy as i32;
                            let c = buffer[y as usize * self.width + x as usize];

                            let c = GrayScalePixel::from_rgb(c.r, c.g, c.b);

                            screen.draw(x as usize, y as usize, c.0);
                        }
                    }

                    // println!("Drawing to: {:?}", damage);

                    if screen.is_updating() {
                        if let Some(r) = accumulated_updates.as_mut() {
                            *r = r.union(&rect_from_phys(damage.clone()));
                        } else {
                            accumulated_updates = Some(rect_from_phys(damage.clone()));
                        }

                        if last_draw_at.elapsed() > Duration::from_millis(20) {
                            let redraw_region = accumulated_updates.take().unwrap();
                            screen.dynamic_update(
                                redraw_region.origin.x,
                                redraw_region.origin.y,
                                redraw_region.width() as u32,
                                redraw_region.height() as u32,
                            );
                            last_draw_at = Instant::now();
                        }

                        if let Some(r) = dynamic_region_to_redraw.as_mut() {
                            *r = r.union(&rect_from_phys(damage));
                        } else {
                            dynamic_region_to_redraw = Some(rect_from_phys(damage));
                        }

                        fulfill_dynamic_updates_after =
                            Some(Instant::now() + Duration::from_millis(200));
                    } else {
                        screen.partial_update(
                            damage.bounding_box_origin().x,
                            damage.bounding_box_origin().y,
                            damage.bounding_box_size().width,
                            damage.bounding_box_size().height,
                        );
                        last_draw_at = Instant::now();
                    }
                });
            }
        }
    }
}

fn ink_key_to_slint(key: Key) -> Option<slint::platform::Key> {
    match key {
        Key::Up => Some(slint::platform::Key::UpArrow),
        Key::Down => Some(slint::platform::Key::DownArrow),
        Key::Left => Some(slint::platform::Key::LeftArrow),
        Key::Prev => Some(slint::platform::Key::LeftArrow),
        Key::Prev2 => Some(slint::platform::Key::LeftArrow),
        Key::Right => Some(slint::platform::Key::RightArrow),
        Key::Next => Some(slint::platform::Key::RightArrow),
        Key::Next2 => Some(slint::platform::Key::RightArrow),
        Key::Ok => Some(slint::platform::Key::Return),
        Key::Back => Some(slint::platform::Key::Backspace),
        Key::Menu => Some(slint::platform::Key::Menu),
        Key::Home => Some(slint::platform::Key::Home),
        Key::Plus => Some(slint::platform::Key::PageUp),
        Key::Minus => Some(slint::platform::Key::PageDown),
        _ => None,
    }
}

fn ink_evt_to_slint(evt: Event) -> Option<WindowEvent> {
    println!("evt: {:?}", evt);
    let evt = match evt {
        Event::PointerDown { x, y } => WindowEvent::PointerPressed {
            position: slint::PhysicalPosition { x, y }.to_logical(SCALE_FACTOR),
            button: slint::platform::PointerEventButton::Left,
        },
        Event::PointerMove { x, y } => WindowEvent::PointerMoved {
            position: slint::PhysicalPosition { x, y }.to_logical(SCALE_FACTOR),
        },
        Event::PointerUp { x, y } => WindowEvent::PointerReleased {
            position: slint::PhysicalPosition { x, y }.to_logical(SCALE_FACTOR),
            button: slint::platform::PointerEventButton::Left,
        },
        Event::Foreground { .. } => WindowEvent::WindowActiveChanged(true),
        Event::Background { .. } => WindowEvent::WindowActiveChanged(false),
        Event::KeyDown { key } => {
            if let Some(slint_key) = ink_key_to_slint(key) {
                WindowEvent::KeyPressed {
                    text: slint_key.into(),
                }
            } else {
                return None;
            }
        }
        Event::KeyRepeat { key } => {
            if let Some(slint_key) = ink_key_to_slint(key) {
                WindowEvent::KeyPressRepeated {
                    text: slint_key.into(),
                }
            } else {
                return None;
            }
        }
        Event::KeyUp { key } => {
            if let Some(slint_key) = ink_key_to_slint(key) {
                WindowEvent::KeyReleased {
                    text: slint_key.into(),
                }
            } else {
                return None;
            }
        }
        _ => return None,
    };

    Some(evt)
}
