#![deny(clippy::all)]

use std::default;

use napi::bindgen_prelude::Undefined;
use tao::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  platform::macos::WindowBuilderExtMacOS,
  window::{Fullscreen, WindowBuilder},
};
use wry::WebViewBuilder;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi(object)]
#[derive(Debug)]
#[non_exhaustive]

pub struct OpenWebpageOptions {
  pub url: Option<String>,
  pub fullscreen: Option<bool>,
}

#[napi]
pub fn open_webpage(_options: Option<OpenWebpageOptions>) -> Undefined {
  let options = OpenWebpageOptions {
    url: Some("https://google.com".to_string()),
    fullscreen: Some(false),
    .._options.unwrap()
  };

  // let options = options_.unwrap_or_default();
  println!("Opening webpage:  {:?}", options);

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    // .with_transparent(true)
    // .with_titlebar_transparent(true)
    // .with_visible_on_all_workspaces(true)
    // .with_always_on_top(true)
    .with_fullscreen(match options.fullscreen.unwrap() {
      true => Some(Fullscreen::Borderless(None)),
      false => None,
    })
    .build(&event_loop)
    .unwrap();

  let builder = WebViewBuilder::new().with_url(options.url.unwrap());

  #[cfg(not(target_os = "linux"))]
  let webview = builder.build(&window).unwrap();
  #[cfg(target_os = "linux")]
  let webview = builder.build_gtk(window.gtk_window()).unwrap();

  event_loop.run(move |event, _, control_flow| {
    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    *control_flow = ControlFlow::Poll;

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    *control_flow = ControlFlow::Wait;

    match event {
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => {
        println!("The close button was pressed; stopping");
        *control_flow = ControlFlow::Exit
      }
      Event::MainEventsCleared => {
        // Application update code.

        // Queue a RedrawRequested event.
        //
        // You only need to call this if you've determined that you need to redraw, in
        // applications which do not always need to. Applications that redraw continuously
        // can just render here instead.
        window.request_redraw();
      }
      Event::RedrawRequested(_) => {
        // Redraw the application.
        //
        // It's preferable for applications that do not render continuously to render in
        // this event rather than in MainEventsCleared, since rendering in here allows
        // the program to gracefully handle redraws requested by the OS.
      }
      _ => (),
    }
  });

  ()
}
