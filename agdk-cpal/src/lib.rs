use std::sync::mpsc::channel;
use std::time::Duration;
///! Based on https://github.com/RustAudio/cpal/blob/master/examples/android.rs
use android_activity::{AndroidApp, InputStatus, MainEvent, PollEvent};
use log::{info, LevelFilter};

use knyst::{
    audio_backend::{CpalBackend, CpalBackendOptions},
    prelude::*,
};

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_max_level(LevelFilter::Trace));

    let mut quit = false;
    let mut redraw_pending = true;
    let mut render_state: Option<()> = Default::default();

    let (error_sender, _error_receiver) = channel();
    let mut backend = CpalBackend::new(CpalBackendOptions::default()).expect("Cpal backend creation failed");
    let _sphere = KnystSphere::start(
        &mut backend,
        SphereSettings {
            num_inputs: 0,
            num_outputs: 1,
            ..Default::default()
        },
        Box::new(move |error| {
            error_sender.send(format!("{error}")).unwrap();
        }),
    );

    let sample_rate = backend.sample_rate() as f32;
    let block_size = backend.block_size().unwrap_or(64);

    let mut k = knyst_commands();

    let mut graph_settings = k.default_graph_settings();
    graph_settings.sample_rate = sample_rate;
    graph_settings.block_size = block_size;
    graph_settings.num_outputs = 2;
    graph_settings.num_inputs = 0;
    let mut graph = Graph::new(graph_settings);

    let amp = graph.push(Mult);

    graph.connect(amp.to_graph_out().channels(2)).unwrap();

    let noise_node: NodeId;

    let noise = PinkNoise::new();
    noise_node = graph.push(noise);

    graph
        .connect(noise_node.to(amp).to_index(0))
        .unwrap();

    graph
        .connect(
            constant(0.5)
                .to(amp)
                .to_index(1),
        )
        .unwrap();

    graph.connect(amp.to_graph_out().channels(2)).unwrap();

    let graph_id = k.push(graph, inputs!());
    k.connect(graph_id.to_graph_out().channels(2));

    let total_duration = 64;
    for _ in 0..total_duration {
        std::thread::sleep(Duration::from_secs(1));
    }
    std::thread::sleep(Duration::from_secs(1));

    while !quit {
        app.poll_events(
            Some(std::time::Duration::from_millis(500)), /* timeout */
            |event| {
                match event {
                    PollEvent::Wake => {
                        info!("Early wake up");
                    }
                    PollEvent::Timeout => {
                        info!("Timed out");
                        // Real app would probably rely on vblank sync via graphics API...
                        redraw_pending = true;
                    }
                    PollEvent::Main(main_event) => {
                        info!("Main event: {:?}", main_event);
                        match main_event {
                            MainEvent::SaveState { saver, .. } => {
                                saver.store("foo://bar".as_bytes());
                            }
                            MainEvent::InitWindow { .. } => {
                                render_state = Some(());
                                redraw_pending = true;
                            }
                            MainEvent::TerminateWindow { .. } => {
                                render_state = None;
                            }
                            MainEvent::WindowResized { .. } => {
                                redraw_pending = true;
                            }
                            MainEvent::RedrawNeeded { .. } => {
                                redraw_pending = true;
                            }
                            MainEvent::LowMemory => {}

                            MainEvent::Destroy => quit = true,
                            _ => { /* ... */ }
                        }
                    }
                    _ => {}
                }

                // if redraw_pending {
                //     if let Some(_rs) = render_state {
                //         redraw_pending = false;
                //
                //         // Handle input
                //         app.input_events(|event| {
                //             info!("Input Event: {event:?}");
                //             InputStatus::Unhandled
                //         });
                //
                //         info!("Render...");
                //     }
                // }
            },
        );
    }
}
