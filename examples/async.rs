//! A Bevy GUI application that communicates asynchronously with a background
//! task simulating a NetworkManager service. It demonstrates how to:
//!
//! - Send messages from the GUI to async code using `EventReader` & `EventWriter`.
//! - Listen to async responses using `mpsc::Receiver` in a task pool.
//! - Update UI reactively using Bevy resources and change detection.
//!
//! Uses `tokio` for async messaging.

use std::time::Duration;

use bevy::{ecs::prelude::*, prelude::*, tasks::AsyncComputeTaskPool, winit::WinitSettings};
use bevy_styled_widgets::{
    StyledWidgetsPlugin,
    prelude::{StyledButton, ThemeManager},
};
use tokio::{select, sync::mpsc};

/// Represents the current state of the wireless device in GUI.
#[derive(Debug, Default, PartialEq, Eq)]
pub enum WirelessStatus {
    On,
    #[default]
    Off,
    Connected,
}

/// Bevy `Resource` to track the wireless state.
#[derive(Resource, Default)]
pub struct Wireless {
    status: WirelessStatus,
}

/// Events that can be sent from GUI systems to trigger async behavior.
#[derive(Event)]
pub enum Messages {
    ToggleWireless,
}

/// Holds channels to communicate (to and fro) with the async `NetworkManager` task.
/// nm_tx is for sending the messages from GUI to network manager
/// nm_event_rx is receiver, which will get the messages when wireless state changes
#[derive(Resource)]
pub struct AppResources {
    nm_tx: mpsc::Sender<NetworkManagerRequest>,
    nm_event_rx: mpsc::Receiver<Result<WifiState>>,
}

/// System to listen for Bevy GUI events and trigger async tasks accordingly.
fn listen_bevy_event_stream(
    mut stream: EventReader<Messages>,
    mut wireless: ResMut<Wireless>,
    app_resources: Res<AppResources>,
) {
    for event in stream.read() {
        match event {
            Messages::ToggleWireless => {
                println!("listen_bevy_event_stream() toggle wireless");

                let nm_tx = app_resources.nm_tx.clone();
                let pool = AsyncComputeTaskPool::get();
                let reply_to = mpsc::channel(1).0;

                // Based on the current state, send a different async request.
                let request = match wireless.status {
                    WirelessStatus::On | WirelessStatus::Connected => {
                        wireless.status = WirelessStatus::Off;
                        NetworkManagerRequest::DisableWirelessDevice { reply_to }
                    }
                    WirelessStatus::Off => {
                        wireless.status = WirelessStatus::On;
                        NetworkManagerRequest::EnableWirelessDevice { reply_to }
                    }
                };

                // Send the request to the async handler.
                pool.spawn(async move {
                    let _ = nm_tx.send(request).await;
                })
                .detach();
            }
        };
    }
}

/// System to initialize async NetworkManager service and spawn its handler.
fn start_service(mut commands: Commands) {
    let (nm_tx, nm_rx) = mpsc::channel(10);
    let (reply_to, nm_event_rx) = mpsc::channel(1);
    let pool = AsyncComputeTaskPool::get();
    let nm_tx_2 = nm_tx.clone();

    // Start the async handler for incoming NetworkManager requests.
    pool.spawn(async move {
        let nm_handler = Client::new().await;
        let _ = nm_handler.run(nm_rx).await;
    })
    .detach();

    // Subscribe to wifi state change notifications.
    pool.spawn(async move {
        let request = NetworkManagerRequest::GetDeviceStateChangeEvent { reply_to };
        let _ = nm_tx_2.send(request).await;
    })
    .detach();

    // Store the channels in a Bevy resource for later use.
    commands.insert_resource(AppResources { nm_tx, nm_event_rx });
}

/// Listens for wifi state change messages from async code and updates GUI state.
fn listen_wireless_service_events(
    mut app_resources: ResMut<AppResources>,
    mut wireless: ResMut<Wireless>,
) {
    if let Ok(res) = app_resources.nm_event_rx.try_recv() {
        if let Ok(wifi_state) = res {
            println!("got wireless event {:?}", wifi_state);

            // Update the wireless status based on the incoming state.
            wireless.status = match wifi_state {
                WifiState::Connecting => WirelessStatus::On,
                WifiState::Connected => WirelessStatus::Connected,
                WifiState::Disconnected | WifiState::Disconnecting | WifiState::Unknown => {
                    WirelessStatus::Off
                }
            };
        }
    }
}

/// Represents an async network manager service client that handles incoming messages.
pub struct Client {}

impl Client {
    /// Create a new client.
    pub async fn new() -> Self {
        Self {}
    }

    /// Simulated NetworkManager request handler loop.
    pub async fn run(&self, mut nm_request: mpsc::Receiver<NetworkManagerRequest>) {
        loop {
            select! {
                msg = nm_request.recv() => {
                    if let Some(message) = msg {
                        match message {
                            NetworkManagerRequest::EnableWirelessDevice { reply_to } => {
                                println!("server: enable wireless");
                                let _ = reply_to.send(Ok(())).await;
                            },
                            NetworkManagerRequest::DisableWirelessDevice { reply_to } => {
                                println!("server: disable wireless");
                                let _ = reply_to.send(Ok(())).await;
                            },
                            NetworkManagerRequest::GetDeviceStateChangeEvent { reply_to } => {
                                println!("server: subscribe wireless");
                                async_std::task::sleep(Duration::from_secs(3)).await;
                                let _ = reply_to.send(Ok(WifiState::Connected)).await;
                            },
                        }
                    }
                }
            }
        }
    }
}

/// Enum representing async requests to the `Client`.
#[derive(Debug)]
pub enum NetworkManagerRequest {
    EnableWirelessDevice {
        reply_to: mpsc::Sender<Result<()>>,
    },
    DisableWirelessDevice {
        reply_to: mpsc::Sender<Result<()>>,
    },
    GetDeviceStateChangeEvent {
        reply_to: mpsc::Sender<Result<WifiState>>,
    },
}

/// Various states the wifi device can be in.
/// Server side
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WifiState {
    Connecting,
    Connected,
    Disconnected,
    Disconnecting,
    Unknown,
}

/// Marker component for identifying the button that displays wireless status.
#[derive(Component)]
pub struct WirelessComponent;

/// Main Bevy app entry point.
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StyledWidgetsPlugin))
        .insert_resource(ThemeManager::default())
        .insert_resource(WinitSettings::desktop_app())
        .add_event::<Messages>()
        .insert_resource(Wireless::default())
        .add_systems(Startup, (start_service, setup_view).chain())
        .add_systems(Update, listen_bevy_event_stream)
        .add_systems(Update, update_ui.run_if(resource_changed::<Wireless>))
        .add_systems(Update, listen_wireless_service_events)
        .run();
}

/// Updates the UI text of the wireless button based on the `Wireless` resource.
fn update_ui(
    mut q_wireless_component: Query<&mut StyledButton, With<WirelessComponent>>,
    wireless: Res<Wireless>,
) {
    println!("update ui");
    for mut button in q_wireless_component.iter_mut() {
        let status = match wireless.status {
            WirelessStatus::On => "on",
            WirelessStatus::Off => "off",
            WirelessStatus::Connected => "connected",
        };
        button.text = Some(status.to_string());
    }
}

/// Bevy callback that sends a `ToggleWireless` event to the system.
fn toggle_wireless(wireless: Res<Wireless>, mut stream: EventWriter<Messages>) {
    println!("toggle_wireless()");
    stream.write(Messages::ToggleWireless);
}

/// Sets up the UI layout with a single styled button.
fn setup_view(mut commands: Commands, wireless: Res<Wireless>) {
    println!("setup_view()");

    let toggle_wireless = commands.register_system(toggle_wireless);

    commands.spawn(Camera2d);

    let status = match wireless.status {
        WirelessStatus::On => "on",
        WirelessStatus::Off => "off",
        WirelessStatus::Connected => "connected",
    };

    commands.spawn((
        StyledButton::builder()
            .text(status)
            .on_click(toggle_wireless)
            .build(),
        WirelessComponent,
    ));
}
