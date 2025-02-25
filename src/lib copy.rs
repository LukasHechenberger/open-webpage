#![deny(clippy::all)]

use futures::prelude::*;
use std::str;

use napi::{bindgen_prelude::*, CallContext, JsNumber, JsString, Task};
use tao::{
  event::{Event, WindowEvent},
  event_loop::{self, ControlFlow, EventLoop},
  platform::{macos::WindowBuilderExtMacOS, run_return::EventLoopExtRunReturn},
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
        .unwrap_or("https://google.com".to_string()),
      title: partial.title.clone().unwrap_or("open-webpage".to_string()),
      fullscreen: partial.fullscreen.unwrap_or(false),
    }
  }
}

pub fn open_webpage_with_options(options: ResolvedOpenWebpageOptions) -> EventLoop<()> {
  println!("Opening webpage:  {:?}", options);

  let mut event_loop = EventLoop::new();
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

  #[cfg(not(target_os = "linux"))]
  let webview = builder.build(&window).unwrap();
  #[cfg(target_os = "linux")]
  let webview = builder.build_gtk(window.gtk_window()).unwrap();

  return event_loop;

  // event_loop.run_return(move |event, _, control_flow| {
  //   // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
  //   // dispatched any events. This is ideal for games and similar applications.
  //   *control_flow = ControlFlow::Wait;

  //   // ControlFlow::Wait pauses the event loop if no events are available to process.
  //   // This is ideal for non-game applications that only update in response to user
  //   // input, and uses significantly less power/CPU time than ControlFlow::Poll.
  //   *control_flow = ControlFlow::Wait;

  //   match event {
  //     Event::WindowEvent {
  //       event: WindowEvent::CloseRequested,
  //       ..
  //     } => {
  //       println!("The close button was pressed; stopping");
  //       *control_flow = ControlFlow::Exit
  //     }
  //     Event::MainEventsCleared => {
  //       // Application update code.

  //       // Queue a RedrawRequested event.
  //       //
  //       // You only need to call this if you've determined that you need to redraw, in
  //       // applications which do not always need to. Applications that redraw continuously
  //       // can just render here instead.
  //       window.request_redraw();
  //     }
  //     Event::RedrawRequested(_) => {
  //       // Redraw the application.
  //       //
  //       // It's preferable for applications that do not render continuously to render in
  //       // this event rather than in MainEventsCleared, since rendering in here allows
  //       // the program to gracefully handle redraws requested by the OS.
  //     }
  //     _ => (),
  //   }
  // });

  // Ok(Undefined::default())
}

// #[napi]
// pub fn open_webpage(options: OpenWebpageOptions) -> Undefined {
//   open_webpage_with_options(ResolvedOpenWebpageOptions::from_partial(&options));
// }

fn start_event_loop(event_loop: &EventLoop<()>) -> Result<Undefined> {
  return Ok(Undefined::default());
}

// struct EventLoopTask {
//   event_loop: EventLoop<()>,
// }

// // #[napi(js_name = "Page")]
// pub struct JsEventLoopTask {
//   task: EventLoopTask,
// }

// impl JsEventLoopTask {
//   pub fn new(event_loop: EventLoop<()>) -> Self {
//     Self {
//       task: EventLoopTask { event_loop },
//     }
//   }
// }

// impl Task for JsEventLoopTask {
//   type Output = Undefined;
//   type JsValue = Undefined;

//   fn compute(&mut self) -> Result<Self::Output> {
//     Ok(start_event_loop(&self.event_loop))
//   }

//   fn resolve(&mut self) -> Result<Self::JsValue> {
//     Ok(Undefined::default())
//   }
// }

pub fn fib(n: u32) -> u32 {
  if n <= 1 {
    return n;
  }

  fib(n - 1) + fib(n - 2)
}

pub struct AsyncFib {
  input: &EventLoop<()>,
}

impl Task for AsyncFib {
  type Output = u32;
  type JsValue = JsNumber;

  fn compute(&mut self) -> Result<Self::Output> {
    // let event_loop = EventLoop::new();

    Ok(fib(self.input))
  }

  fn resolve(&mut self, env: Env, output: u32) -> Result<Self::JsValue> {
    env.create_uint32(output)
  }
}

pub async fn wait_for_loop(event_loop: &EventLoop<()>) -> Result<Undefined> {
  // let v = p.await?;

  Ok(())
}

// #[napi]
// pub fn open_webpage_async(input: u32, signal: Option<AbortSignal>) -> FutResult<Undefined> {
//   let event_loop = EventLoop::new();
//   println!("Computing fib({:?})", event_loop);

//   // AsyncTask::with_optional_signal(AsyncFib { input }, signal)
//   wait_for_loop(&event_loop)
// }

#[js_function(1)]
pub fn another(ctx: CallContext) -> Result<JsNumber> {
  let js_filepath = ctx.get::<JsString>(0)?;
  let path_str = js_filepath.into_utf8()?.into_owned()?;
  ctx.env.execute_tokio_future(
    tokio::fs::read(path_str).map(|v| {
      v.map_err(|e| {
        Error::new(
          Status::GenericFailure,
          format!("failed to read file, {}", e),
        )
      })
    }),
    |&mut env, data| env.create_buffer_with_data(data).map(|v| v.into_raw()),
  )
}

#[napi]
pub fn open_webpage(options: OpenWebpageOptions) -> Result<Undefined> {
  let event_loop = open_webpage_with_options(ResolvedOpenWebpageOptions::from_partial(&options));

  start_event_loop(&event_loop)
}
