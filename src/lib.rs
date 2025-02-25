#![deny(clippy::all)]
use std::str;

use napi::bindgen_prelude::*;
use tao::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{Fullscreen, WindowBuilder},
};
use wry::WebViewBuilder;

#[macro_use]
extern crate napi_derive;

#[napi(object)]
#[derive(Debug, Default)]
pub struct OpenWebpageOptions {
  pub url: Option<String>,
  pub title: Option<String>,
  pub fullscreen: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ResolvedOpenWebpageOptions {
  pub url: String,
  pub title: String,
  pub fullscreen: bool,
}

impl ResolvedOpenWebpageOptions {
  pub fn from_partial(partial: &OpenWebpageOptions) -> Self {
    Self {
      url: partial
        .url
        .clone()
        .unwrap_or("https://exmaple.com".to_string()),
      title: partial.title.clone().unwrap_or("open-webpage".to_string()),
      fullscreen: partial.fullscreen.unwrap_or(false),
    }
  }
}

pub fn open_webpage_with_options(options: ResolvedOpenWebpageOptions) {
  println!("Opening webpage:  {:?}", options);

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_title(options.title)
    // .with_transparent(true)
    // .with_titlebar_transparent(true)
    // .with_visible_on_all_workspaces(true)
    // .with_always_on_top(true)
    .with_fullscreen(match options.fullscreen {
      true => Some(Fullscreen::Borderless(None)),
      false => None,
    })
    .build(&event_loop)
    .unwrap();

  let builder = WebViewBuilder::new().with_url(options.url);
  let _webview = builder.build(&window).unwrap();

  event_loop.run(move |event, _, control_flow| {
    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    *control_flow = ControlFlow::Wait;

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    *control_flow = ControlFlow::Wait;

    match event {
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => {
        println!("The close button was pressed, stopping...");
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
}

#[napi]
pub fn open_webpage(options: Option<OpenWebpageOptions>) -> Result<Undefined> {
  open_webpage_with_options(ResolvedOpenWebpageOptions::from_partial(&options.unwrap()));

  Ok(())
}
