//! The [`Event`] enum and assorted supporting types.
//!
//! These are sent to the closure given to [`EventLoop::run(...)`], where they get
//! processed and used to modify the program state. For more details, see the root-level documentation.
//!
//! Some of these events represent different "parts" of a traditional event-handling loop. You could
//! approximate the basic ordering loop of [`EventLoop::run(...)`] like this:
//!
//! ```rust,ignore
//! let mut start_cause = StartCause::Init;
//!
//! while !elwt.exiting() {
//!     event_handler(NewEvents(start_cause), elwt);
//!
//!     for e in (window events, user events, device events) {
//!         event_handler(e, elwt);
//!     }
//!
//!     for w in (redraw windows) {
//!         event_handler(RedrawRequested(w), elwt);
//!     }
//!
//!     event_handler(AboutToWait, elwt);
//!     start_cause = wait_if_necessary();
//! }
//!
//! event_handler(LoopExiting, elwt);
//! ```
//!
//! This leaves out timing details like [`ControlFlow::WaitUntil`] but hopefully
//! describes what happens in what order.
//!
//! [`EventLoop::run(...)`]: crate::event_loop::EventLoop::run
//! [`ControlFlow::WaitUntil`]: crate::event_loop::ControlFlow::WaitUntil

use lite_macro::lite_api;

use crate::lite_math::{PodVector2, PodVector2i};
#[cfg(doc)]
use crate::window::Window;
use fyrox::dpi::{PhysicalPosition, PhysicalSize};

// type Instant = i64;

/// Describes a generic event.
///
/// See the module-level docs for more information on the event loop manages each event.
#[derive(Debug, Clone, PartialEq)]
#[lite_api]
pub enum Event {
    /// Emitted when the OS sends an event to a winit window.
    WindowEvent { window_id: i64, event: WindowEvent },

    /// Emitted when the OS sends an event to a device.
    DeviceEvent { event: DeviceEvent },

    /// Emitted when the application has been suspended.
    ///
    /// # Portability
    ///
    /// Not all platforms support the notion of suspending applications, and there may be no
    /// technical way to guarantee being able to emit a `Suspended` event if the OS has
    /// no formal application lifecycle (currently only Android, iOS, and Web do). For this reason,
    /// Winit does not currently try to emit pseudo `Suspended` events before the application
    /// quits on platforms without an application lifecycle.
    ///
    /// Considering that the implementation of `Suspended` and [`Resumed`] events may be internally
    /// driven by multiple platform-specific events, and that there may be subtle differences across
    /// platforms with how these internal events are delivered, it's recommended that applications
    /// be able to gracefully handle redundant (i.e. back-to-back) `Suspended` or [`Resumed`] events.
    ///
    /// Also see [`Resumed`] notes.
    ///
    /// ## Android
    ///
    /// On Android, the `Suspended` event is only sent when the application's associated
    /// [`SurfaceView`] is destroyed. This is expected to closely correlate with the [`onPause`]
    /// lifecycle event but there may technically be a discrepancy.
    ///
    /// [`onPause`]: https://developer.android.com/reference/android/app/Activity#onPause()
    ///
    /// Applications that need to run on Android should assume their [`SurfaceView`] has been
    /// destroyed, which indirectly invalidates any existing render surfaces that may have been
    /// created outside of Winit (such as an `EGLSurface`, [`VkSurfaceKHR`] or [`wgpu::Surface`]).
    ///
    /// After being `Suspended` on Android applications must drop all render surfaces before
    /// the event callback completes, which may be re-created when the application is next [`Resumed`].
    ///
    /// [`SurfaceView`]: https://developer.android.com/reference/android/view/SurfaceView
    /// [Activity lifecycle]: https://developer.android.com/guide/components/activities/activity-lifecycle
    /// [`VkSurfaceKHR`]: https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceKHR.html
    /// [`wgpu::Surface`]: https://docs.rs/wgpu/latest/wgpu/struct.Surface.html
    ///
    /// ## iOS
    ///
    /// On iOS, the `Suspended` event is currently emitted in response to an
    /// [`applicationWillResignActive`] callback which means that the application is
    /// about to transition from the active to inactive state (according to the
    /// [iOS application lifecycle]).
    ///
    /// [`applicationWillResignActive`]: https://developer.apple.com/documentation/uikit/uiapplicationdelegate/1622950-applicationwillresignactive
    /// [iOS application lifecycle]: https://developer.apple.com/documentation/uikit/app_and_environment/managing_your_app_s_life_cycle
    ///
    /// ## Web
    ///
    /// On Web, the `Suspended` event is emitted in response to a [`pagehide`] event
    /// with the property [`persisted`] being true, which means that the page is being
    /// put in the [`bfcache`] (back/forward cache) - an in-memory cache that stores a
    /// complete snapshot of a page (including the JavaScript heap) as the user is
    /// navigating away.
    ///
    /// [`pagehide`]: https://developer.mozilla.org/en-US/docs/Web/API/Window/pagehide_event
    /// [`persisted`]: https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/persisted
    /// [`bfcache`]: https://web.dev/bfcache/
    ///
    /// [`Resumed`]: Self::Resumed
    Suspended,

    /// Emitted when the application has been resumed.
    ///
    /// For consistency, all platforms emit a `Resumed` event even if they don't themselves have a
    /// formal suspend/resume lifecycle. For systems without a standard suspend/resume lifecycle
    /// the `Resumed` event is always emitted after the [`NewEvents(StartCause::Init)`][StartCause::Init]
    /// event.
    ///
    /// # Portability
    ///
    /// It's recommended that applications should only initialize their graphics context and create
    /// a window after they have received their first `Resumed` event. Some systems
    /// (specifically Android) won't allow applications to create a render surface until they are
    /// resumed.
    ///
    /// Considering that the implementation of [`Suspended`] and `Resumed` events may be internally
    /// driven by multiple platform-specific events, and that there may be subtle differences across
    /// platforms with how these internal events are delivered, it's recommended that applications
    /// be able to gracefully handle redundant (i.e. back-to-back) [`Suspended`] or `Resumed` events.
    ///
    /// Also see [`Suspended`] notes.
    ///
    /// ## Android
    ///
    /// On Android, the `Resumed` event is sent when a new [`SurfaceView`] has been created. This is
    /// expected to closely correlate with the [`onResume`] lifecycle event but there may technically
    /// be a discrepancy.
    ///
    /// [`onResume`]: https://developer.android.com/reference/android/app/Activity#onResume()
    ///
    /// Applications that need to run on Android must wait until they have been `Resumed`
    /// before they will be able to create a render surface (such as an `EGLSurface`,
    /// [`VkSurfaceKHR`] or [`wgpu::Surface`]) which depend on having a
    /// [`SurfaceView`]. Applications must also assume that if they are [`Suspended`], then their
    /// render surfaces are invalid and should be dropped.
    ///
    /// Also see [`Suspended`] notes.
    ///
    /// [`SurfaceView`]: https://developer.android.com/reference/android/view/SurfaceView
    /// [Activity lifecycle]: https://developer.android.com/guide/components/activities/activity-lifecycle
    /// [`VkSurfaceKHR`]: https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceKHR.html
    /// [`wgpu::Surface`]: https://docs.rs/wgpu/latest/wgpu/struct.Surface.html
    ///
    /// ## iOS
    ///
    /// On iOS, the `Resumed` event is emitted in response to an [`applicationDidBecomeActive`]
    /// callback which means the application is "active" (according to the
    /// [iOS application lifecycle]).
    ///
    /// [`applicationDidBecomeActive`]: https://developer.apple.com/documentation/uikit/uiapplicationdelegate/1622956-applicationdidbecomeactive
    /// [iOS application lifecycle]: https://developer.apple.com/documentation/uikit/app_and_environment/managing_your_app_s_life_cycle
    ///
    /// ## Web
    ///
    /// On Web, the `Resumed` event is emitted in response to a [`pageshow`] event
    /// with the property [`persisted`] being true, which means that the page is being
    /// restored from the [`bfcache`] (back/forward cache) - an in-memory cache that
    /// stores a complete snapshot of a page (including the JavaScript heap) as the
    /// user is navigating away.
    ///
    /// [`pageshow`]: https://developer.mozilla.org/en-US/docs/Web/API/Window/pageshow_event
    /// [`persisted`]: https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/persisted
    /// [`bfcache`]: https://web.dev/bfcache/
    ///
    /// [`Suspended`]: Self::Suspended
    Resumed,

    /// Emitted when the event loop is about to block and wait for new events.
    ///
    /// Most applications shouldn't need to hook into this event since there is no real relationship
    /// between how often the event loop needs to wake up and the dispatching of any specific events.
    ///
    /// High frequency event sources, such as input devices could potentially lead to lots of wake
    /// ups and also lots of corresponding `AboutToWait` events.
    ///
    /// This is not an ideal event to drive application rendering from and instead applications
    /// should render in response to [`WindowEvent::RedrawRequested`] events.
    AboutToWait,

    /// Emitted when the event loop is being shut down.
    ///
    /// This is irreversible - if this event is emitted, it is guaranteed to be the last event that
    /// gets emitted. You generally want to treat this as a "do on quit" event.
    LoopExiting,

    /// Emitted when the application has received a memory warning.
    ///
    /// ## Platform-specific
    ///
    /// ### Android
    ///
    /// On Android, the `MemoryWarning` event is sent when [`onLowMemory`] was called. The application
    /// must [release memory] or risk being killed.
    ///
    /// [`onLowMemory`]: https://developer.android.com/reference/android/app/Application.html#onLowMemory()
    /// [release memory]: https://developer.android.com/topic/performance/memory#release
    ///
    /// ### iOS
    ///
    /// On iOS, the `MemoryWarning` event is emitted in response to an [`applicationDidReceiveMemoryWarning`]
    /// callback. The application must free as much memory as possible or risk being terminated, see
    /// [how to respond to memory warnings].
    ///
    /// [`applicationDidReceiveMemoryWarning`]: https://developer.apple.com/documentation/uikit/uiapplicationdelegate/1623063-applicationdidreceivememorywarni
    /// [how to respond to memory warnings]: https://developer.apple.com/documentation/uikit/app_and_environment/managing_your_app_s_life_cycle/responding_to_memory_warnings
    ///
    /// ### Others
    ///
    /// - **macOS / Wayland / Windows / Orbital:** Unsupported.
    MemoryWarning,
}

/// Describes the reason the event loop is resuming.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[lite_api]
pub enum StartCause {
    /// Sent if the time specified by [`ControlFlow::WaitUntil`] has been reached. Contains the
    /// moment the timeout was requested and the requested resume time. The actual resume time is
    /// guaranteed to be equal to or after the requested resume time.
    ///
    /// [`ControlFlow::WaitUntil`]: crate::event_loop::ControlFlow::WaitUntil
    ResumeTimeReached,

    /// Sent if the OS has new events to send to the window, after a wait was requested. Contains
    /// the moment the wait was requested and the resume time, if requested.
    WaitCancelled,

    /// Sent if the event loop is being resumed after the loop's control flow was set to
    /// [`ControlFlow::Poll`].
    ///
    /// [`ControlFlow::Poll`]: crate::event_loop::ControlFlow::Poll
    Poll,

    /// Sent once, immediately after `run` is called. Indicates that the loop was just initialized.
    Init,
}

/// Describes an event from a [`Window`].
#[derive(Debug, Clone, PartialEq)]
#[lite_api]
pub enum WindowEvent {
    /// The activation token was delivered back and now could be used.
    ///
    #[allow(rustdoc::broken_intra_doc_links)]
    /// Delivered in response to [`request_activation_token`].
    ///
    /// [`request_activation_token`]: crate::platform::startup_notify::WindowExtStartupNotify::request_activation_token
    ActivationTokenDone,

    /// The size of the window has changed. Contains the client area's new dimensions.
    Resized(PodVector2i),

    /// The position of the window has changed. Contains the window's new position.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Web / Wayland:** Unsupported.
    Moved(PodVector2i),

    /// The window has been requested to close.
    CloseRequested,

    /// The window has been destroyed.
    Destroyed,

    /// A file has been dropped into the window.
    ///
    /// When the user drops multiple files at once, this event will be emitted for each file
    /// separately.
    DroppedFile(String),

    /// A file is being hovered over the window.
    ///
    /// When the user hovers multiple files at once, this event will be emitted for each file
    /// separately.
    HoveredFile(String),

    /// A file was hovered, but has exited the window.
    ///
    /// There will be a single `HoveredFileCancelled` event triggered even if multiple files were
    /// hovered.
    HoveredFileCancelled,

    /// The window gained or lost focus.
    ///
    /// The parameter is true if the window has gained focus, and false if it has lost focus.
    Focused(bool),

    /// An event from the keyboard has been received.
    ///
    /// ## Platform-specific
    /// - **Windows:** The shift key overrides NumLock. In other words, while shift is held down,
    ///   numpad keys act as if NumLock wasn't active. When this is used, the OS sends fake key
    ///   events which are not marked as `is_synthetic`.
    KeyboardInput {
        event: KeyEvent,

        /// If `true`, the event was generated synthetically by winit
        /// in one of the following circumstances:
        ///
        /// * Synthetic key press events are generated for all keys pressed
        ///   when a window gains focus. Likewise, synthetic key release events
        ///   are generated for all keys pressed when a window goes out of focus.
        ///   ***Currently, this is only functional on X11 and Windows***
        ///
        /// Otherwise, this value is always `false`.
        is_synthetic: bool,
    },

    /// The keyboard modifiers have changed.
    ModifiersChanged,

    /// An event from an input method.
    ///
    /// **Note:** You have to explicitly enable this event using [`Window::set_ime_allowed`].
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Web / Orbital:** Unsupported.
    Ime,

    /// The cursor has moved on the window.
    ///
    /// ## Platform-specific
    ///
    /// - **Web:** Doesn't take into account CSS [`border`], [`padding`], or [`transform`].
    ///
    /// [`border`]: https://developer.mozilla.org/en-US/docs/Web/CSS/border
    /// [`padding`]: https://developer.mozilla.org/en-US/docs/Web/CSS/padding
    /// [`transform`]: https://developer.mozilla.org/en-US/docs/Web/CSS/transform
    CursorMoved {
        /// (x,y) coords in pixels relative to the top-left corner of the window. Because the range of this data is
        /// limited by the display area and it may have been transformed by the OS to implement effects such as cursor
        /// acceleration, it should not be used to implement non-cursor-like interactions such as 3D camera control.
        position: PodVector2i,
    },

    /// The cursor has entered the window.
    ///
    /// ## Platform-specific
    ///
    /// - **Web:** Doesn't take into account CSS [`border`], [`padding`], or [`transform`].
    ///
    /// [`border`]: https://developer.mozilla.org/en-US/docs/Web/CSS/border
    /// [`padding`]: https://developer.mozilla.org/en-US/docs/Web/CSS/padding
    /// [`transform`]: https://developer.mozilla.org/en-US/docs/Web/CSS/transform
    CursorEntered,

    /// The cursor has left the window.
    ///
    /// ## Platform-specific
    ///
    /// - **Web:** Doesn't take into account CSS [`border`], [`padding`], or [`transform`].
    ///
    /// [`border`]: https://developer.mozilla.org/en-US/docs/Web/CSS/border
    /// [`padding`]: https://developer.mozilla.org/en-US/docs/Web/CSS/padding
    /// [`transform`]: https://developer.mozilla.org/en-US/docs/Web/CSS/transform
    CursorLeft,

    /// A mouse wheel movement or touchpad scroll occurred.
    MouseWheel {
        delta: MouseScrollDelta,
        phase: TouchPhase,
    },

    /// An mouse button press has been received.
    MouseInput {
        state: ElementState,
        button: MouseButton,
    },

    /// Touchpad magnification event with two-finger pinch gesture.
    ///
    /// Positive delta values indicate magnification (zooming in) and
    /// negative delta values indicate shrinking (zooming out).
    ///
    /// ## Platform-specific
    ///
    /// - Only available on **macOS**.
    TouchpadMagnify { delta: f64, phase: TouchPhase },

    /// Smart magnification event.
    ///
    /// On a Mac, smart magnification is triggered by a double tap with two fingers
    /// on the trackpad and is commonly used to zoom on a certain object
    /// (e.g. a paragraph of a PDF) or (sort of like a toggle) to reset any zoom.
    /// The gesture is also supported in Safari, Pages, etc.
    ///
    /// The event is general enough that its generating gesture is allowed to vary
    /// across platforms. It could also be generated by another device.
    ///
    /// Unfortunatly, neither [Windows](https://support.microsoft.com/en-us/windows/touch-gestures-for-windows-a9d28305-4818-a5df-4e2b-e5590f850741)
    /// nor [Wayland](https://wayland.freedesktop.org/libinput/doc/latest/gestures.html)
    /// support this gesture or any other gesture with the same effect.
    ///
    /// ## Platform-specific
    ///
    /// - Only available on **macOS 10.8** and later.
    SmartMagnify,

    /// Touchpad rotation event with two-finger rotation gesture.
    ///
    /// Positive delta values indicate rotation counterclockwise and
    /// negative delta values indicate rotation clockwise.
    ///
    /// ## Platform-specific
    ///
    /// - Only available on **macOS**.
    TouchpadRotate { delta: f32, phase: TouchPhase },

    /// Touchpad pressure event.
    ///
    /// At the moment, only supported on Apple forcetouch-capable macbooks.
    /// The parameters are: pressure level (value between 0 and 1 representing how hard the touchpad
    /// is being pressed) and stage (integer representing the click level).
    TouchpadPressure { pressure: f32, stage: i64 },

    /// Motion on some analog axis. May report data redundant to other, more specific events.
    AxisMotion { axis: i32, value: f64 },

    /// Touch event has been received
    ///
    /// ## Platform-specific
    ///
    /// - **Web:** Doesn't take into account CSS [`border`], [`padding`], or [`transform`].
    /// - **macOS:** Unsupported.
    ///
    /// [`border`]: https://developer.mozilla.org/en-US/docs/Web/CSS/border
    /// [`padding`]: https://developer.mozilla.org/en-US/docs/Web/CSS/padding
    /// [`transform`]: https://developer.mozilla.org/en-US/docs/Web/CSS/transform
    Touch(Touch),

    /// The window's scale factor has changed.
    ///
    /// The following user actions can cause DPI changes:
    ///
    /// * Changing the display's resolution.
    /// * Changing the display's scale factor (e.g. in Control Panel on Windows).
    /// * Moving the window to a display with a different scale factor.
    ///
    /// After this event callback has been processed, the window will be resized to whatever value
    /// is pointed to by the `new_inner_size` reference. By default, this will contain the size suggested
    /// by the OS, but it can be changed to any value.
    ///
    /// For more information about DPI in general, see the [`dpi`](crate::dpi) module.
    ScaleFactorChanged {
        scale_factor: f64,
        /// Handle to update inner size during scale changes.
        ///
        /// See [`InnerSizeWriter`] docs for more details.
        inner_size_writer: InnerSizeWriter,
    },

    /// The system window theme has changed.
    ///
    /// Applications might wish to react to this to change the theme of the content of the window
    /// when the system changes the window theme.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / X11 / Wayland / Orbital:** Unsupported.
    ThemeChanged,

    /// The window has been occluded (completely hidden from view).
    ///
    /// This is different to window visibility as it depends on whether the window is closed,
    /// minimised, set invisible, or fully occluded by another window.
    ///
    /// ## Platform-specific
    ///
    /// ### iOS
    ///
    /// On iOS, the `Occluded(false)` event is emitted in response to an [`applicationWillEnterForeground`]
    /// callback which means the application should start preparing its data. The `Occluded(true)` event is
    /// emitted in response to an [`applicationDidEnterBackground`] callback which means the application
    /// should free resources (according to the [iOS application lifecycle]).
    ///
    /// [`applicationWillEnterForeground`]: https://developer.apple.com/documentation/uikit/uiapplicationdelegate/1623076-applicationwillenterforeground
    /// [`applicationDidEnterBackground`]: https://developer.apple.com/documentation/uikit/uiapplicationdelegate/1622997-applicationdidenterbackground
    /// [iOS application lifecycle]: https://developer.apple.com/documentation/uikit/app_and_environment/managing_your_app_s_life_cycle
    ///
    /// ### Others
    ///
    /// - **Web:** Doesn't take into account CSS [`border`], [`padding`], or [`transform`].
    /// - **Android / Wayland / Windows / Orbital:** Unsupported.
    ///
    /// [`border`]: https://developer.mozilla.org/en-US/docs/Web/CSS/border
    /// [`padding`]: https://developer.mozilla.org/en-US/docs/Web/CSS/padding
    /// [`transform`]: https://developer.mozilla.org/en-US/docs/Web/CSS/transform
    Occluded(bool),

    /// Emitted when a window should be redrawn.
    ///
    /// This gets triggered in two scenarios:
    /// - The OS has performed an operation that's invalidated the window's contents (such as
    ///   resizing the window).
    /// - The application has explicitly requested a redraw via [`Window::request_redraw`].
    ///
    /// Winit will aggregate duplicate redraw requests into a single event, to
    /// help avoid duplicating rendering work.
    RedrawRequested,
}

/// Identifier of an input device.
///
/// Whenever you receive an event arising from a particular input device, this event contains a `DeviceId` which
/// identifies its origin. Note that devices may be virtual (representing an on-screen cursor and keyboard focus) or
/// physical. Virtual devices typically aggregate inputs from multiple physical devices.
// type DeviceId = i64;

/// Represents raw hardware events that are not associated with any particular window.
///
/// Useful for interactions that diverge significantly from a conventional 2D GUI, such as 3D camera or first-person
/// game controls. Many physical actions, such as mouse movement, can produce both device and window events. Because
/// window events typically arise from virtual devices (corresponding to GUI cursors and keyboard focus) the device IDs
/// may not match.
///
/// Note that these events are delivered regardless of input focus.
#[derive(Clone, Debug, PartialEq)]
#[lite_api]
pub enum DeviceEvent {
    Added,
    Removed,

    /// Change in physical position of a pointing device.
    ///
    /// This represents raw, unfiltered physical motion. Not to be confused with [`WindowEvent::CursorMoved`].
    MouseMotion {
        /// (x, y) change in position in unspecified units.
        ///
        /// Different devices may use different units.
        delta: PodVector2,
    },

    /// Physical scroll event
    MouseWheel {
        delta: MouseScrollDelta,
    },

    /// Motion on some analog axis. This event will be reported for all arbitrary input devices
    /// that winit supports on this platform, including mouse devices.  If the device is a mouse
    /// device then this will be reported alongside the MouseMotion event.
    Motion {
        axis: i32,
        value: f64,
    },

    Button {
        button: i32,
        state: ElementState,
    },

    Key(RawKeyEvent),
}

/// Describes a keyboard input as a raw device event.
///
/// Note that holding down a key may produce repeated `RawKeyEvent`s. The
/// operating system doesn't provide information whether such an event is a
/// repeat or the initial keypress. An application may emulate this by, for
/// example keeping a Map/Set of pressed keys and determining whether a keypress
/// corresponds to an already pressed key.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[lite_api]
pub struct RawKeyEvent {
    pub physical_key: PhysicalKey,
    pub state: ElementState,
}

/// Represents the location of a physical key.
///
/// This type is a superset of [`KeyCode`], including an [`Unidentified`](Self::Unidentified)
/// variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[lite_api]
pub enum PhysicalKey {
    /// A known key code
    Code(KeyCode),
    /// This variant is used when the key cannot be translated to a [`KeyCode`]
    ///
    /// The native keycode is provided (if available) so you're able to more reliably match
    /// key-press and key-release events by hashing the [`PhysicalKey`]. It is also possible to use
    /// this for keybinds for non-standard keys, but such keybinds are tied to a given platform.
    Unidentified(NativeKeyCode),
}

/// Code representing the location of a physical key
///
/// This mostly conforms to the UI Events Specification's [`KeyboardEvent.code`] with a few
/// exceptions:
/// - The keys that the specification calls "MetaLeft" and "MetaRight" are named "SuperLeft" and
///   "SuperRight" here.
/// - The key that the specification calls "Super" is reported as `Unidentified` here.
///
/// [`KeyboardEvent.code`]: https://w3c.github.io/uievents-code/#code-value-tables
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[lite_api]
pub enum KeyCode {
    /// <kbd>`</kbd> on a US keyboard. This is also called a backtick or grave.
    /// This is the <kbd>半角</kbd>/<kbd>全角</kbd>/<kbd>漢字</kbd>
    /// (hankaku/zenkaku/kanji) key on Japanese keyboards
    Backquote,
    /// Used for both the US <kbd>\\</kbd> (on the 101-key layout) and also for the key
    /// located between the <kbd>"</kbd> and <kbd>Enter</kbd> keys on row C of the 102-,
    /// 104- and 106-key layouts.
    /// Labeled <kbd>#</kbd> on a UK (102) keyboard.
    Backslash,
    /// <kbd>[</kbd> on a US keyboard.
    BracketLeft,
    /// <kbd>]</kbd> on a US keyboard.
    BracketRight,
    /// <kbd>,</kbd> on a US keyboard.
    Comma,
    /// <kbd>0</kbd> on a US keyboard.
    Digit0,
    /// <kbd>1</kbd> on a US keyboard.
    Digit1,
    /// <kbd>2</kbd> on a US keyboard.
    Digit2,
    /// <kbd>3</kbd> on a US keyboard.
    Digit3,
    /// <kbd>4</kbd> on a US keyboard.
    Digit4,
    /// <kbd>5</kbd> on a US keyboard.
    Digit5,
    /// <kbd>6</kbd> on a US keyboard.
    Digit6,
    /// <kbd>7</kbd> on a US keyboard.
    Digit7,
    /// <kbd>8</kbd> on a US keyboard.
    Digit8,
    /// <kbd>9</kbd> on a US keyboard.
    Digit9,
    /// <kbd>=</kbd> on a US keyboard.
    Equal,
    /// Located between the left <kbd>Shift</kbd> and <kbd>Z</kbd> keys.
    /// Labeled <kbd>\\</kbd> on a UK keyboard.
    IntlBackslash,
    /// Located between the <kbd>/</kbd> and right <kbd>Shift</kbd> keys.
    /// Labeled <kbd>\\</kbd> (ro) on a Japanese keyboard.
    IntlRo,
    /// Located between the <kbd>=</kbd> and <kbd>Backspace</kbd> keys.
    /// Labeled <kbd>¥</kbd> (yen) on a Japanese keyboard. <kbd>\\</kbd> on a
    /// Russian keyboard.
    IntlYen,
    /// <kbd>a</kbd> on a US keyboard.
    /// Labeled <kbd>q</kbd> on an AZERTY (e.g., French) keyboard.
    KeyA,
    /// <kbd>b</kbd> on a US keyboard.
    KeyB,
    /// <kbd>c</kbd> on a US keyboard.
    KeyC,
    /// <kbd>d</kbd> on a US keyboard.
    KeyD,
    /// <kbd>e</kbd> on a US keyboard.
    KeyE,
    /// <kbd>f</kbd> on a US keyboard.
    KeyF,
    /// <kbd>g</kbd> on a US keyboard.
    KeyG,
    /// <kbd>h</kbd> on a US keyboard.
    KeyH,
    /// <kbd>i</kbd> on a US keyboard.
    KeyI,
    /// <kbd>j</kbd> on a US keyboard.
    KeyJ,
    /// <kbd>k</kbd> on a US keyboard.
    KeyK,
    /// <kbd>l</kbd> on a US keyboard.
    KeyL,
    /// <kbd>m</kbd> on a US keyboard.
    KeyM,
    /// <kbd>n</kbd> on a US keyboard.
    KeyN,
    /// <kbd>o</kbd> on a US keyboard.
    KeyO,
    /// <kbd>p</kbd> on a US keyboard.
    KeyP,
    /// <kbd>q</kbd> on a US keyboard.
    /// Labeled <kbd>a</kbd> on an AZERTY (e.g., French) keyboard.
    KeyQ,
    /// <kbd>r</kbd> on a US keyboard.
    KeyR,
    /// <kbd>s</kbd> on a US keyboard.
    KeyS,
    /// <kbd>t</kbd> on a US keyboard.
    KeyT,
    /// <kbd>u</kbd> on a US keyboard.
    KeyU,
    /// <kbd>v</kbd> on a US keyboard.
    KeyV,
    /// <kbd>w</kbd> on a US keyboard.
    /// Labeled <kbd>z</kbd> on an AZERTY (e.g., French) keyboard.
    KeyW,
    /// <kbd>x</kbd> on a US keyboard.
    KeyX,
    /// <kbd>y</kbd> on a US keyboard.
    /// Labeled <kbd>z</kbd> on a QWERTZ (e.g., German) keyboard.
    KeyY,
    /// <kbd>z</kbd> on a US keyboard.
    /// Labeled <kbd>w</kbd> on an AZERTY (e.g., French) keyboard, and <kbd>y</kbd> on a
    /// QWERTZ (e.g., German) keyboard.
    KeyZ,
    /// <kbd>-</kbd> on a US keyboard.
    Minus,
    /// <kbd>.</kbd> on a US keyboard.
    Period,
    /// <kbd>'</kbd> on a US keyboard.
    Quote,
    /// <kbd>;</kbd> on a US keyboard.
    Semicolon,
    /// <kbd>/</kbd> on a US keyboard.
    Slash,
    /// <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
    AltLeft,
    /// <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
    /// This is labeled <kbd>AltGr</kbd> on many keyboard layouts.
    AltRight,
    /// <kbd>Backspace</kbd> or <kbd>⌫</kbd>.
    /// Labeled <kbd>Delete</kbd> on Apple keyboards.
    Backspace,
    /// <kbd>CapsLock</kbd> or <kbd>⇪</kbd>
    CapsLock,
    /// The application context menu key, which is typically found between the right
    /// <kbd>Super</kbd> key and the right <kbd>Control</kbd> key.
    ContextMenu,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlLeft,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlRight,
    /// <kbd>Enter</kbd> or <kbd>↵</kbd>. Labeled <kbd>Return</kbd> on Apple keyboards.
    Enter,
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
    SuperLeft,
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
    SuperRight,
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftLeft,
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftRight,
    /// <kbd> </kbd> (space)
    Space,
    /// <kbd>Tab</kbd> or <kbd>⇥</kbd>
    Tab,
    /// Japanese: <kbd>変</kbd> (henkan)
    Convert,
    /// Japanese: <kbd>カタカナ</kbd>/<kbd>ひらがな</kbd>/<kbd>ローマ字</kbd> (katakana/hiragana/romaji)
    KanaMode,
    /// Korean: HangulMode <kbd>한/영</kbd> (han/yeong)
    ///
    /// Japanese (Mac keyboard): <kbd>か</kbd> (kana)
    Lang1,
    /// Korean: Hanja <kbd>한</kbd> (hanja)
    ///
    /// Japanese (Mac keyboard): <kbd>英</kbd> (eisu)
    Lang2,
    /// Japanese (word-processing keyboard): Katakana
    Lang3,
    /// Japanese (word-processing keyboard): Hiragana
    Lang4,
    /// Japanese (word-processing keyboard): Zenkaku/Hankaku
    Lang5,
    /// Japanese: <kbd>無変換</kbd> (muhenkan)
    NonConvert,
    /// <kbd>⌦</kbd>. The forward delete key.
    /// Note that on Apple keyboards, the key labelled <kbd>Delete</kbd> on the main part of
    /// the keyboard is encoded as [`Backspace`].
    ///
    /// [`Backspace`]: Self::Backspace
    Delete,
    /// <kbd>Page Down</kbd>, <kbd>End</kbd>, or <kbd>↘</kbd>
    End,
    /// <kbd>Help</kbd>. Not present on standard PC keyboards.
    Help,
    /// <kbd>Home</kbd> or <kbd>↖</kbd>
    Home,
    /// <kbd>Insert</kbd> or <kbd>Ins</kbd>. Not present on Apple keyboards.
    Insert,
    /// <kbd>Page Down</kbd>, <kbd>PgDn</kbd>, or <kbd>⇟</kbd>
    PageDown,
    /// <kbd>Page Up</kbd>, <kbd>PgUp</kbd>, or <kbd>⇞</kbd>
    PageUp,
    /// <kbd>↓</kbd>
    ArrowDown,
    /// <kbd>←</kbd>
    ArrowLeft,
    /// <kbd>→</kbd>
    ArrowRight,
    /// <kbd>↑</kbd>
    ArrowUp,
    /// On the Mac, this is used for the numpad <kbd>Clear</kbd> key.
    NumLock,
    /// <kbd>0 Ins</kbd> on a keyboard. <kbd>0</kbd> on a phone or remote control
    Numpad0,
    /// <kbd>1 End</kbd> on a keyboard. <kbd>1</kbd> or <kbd>1 QZ</kbd> on a phone or remote control
    Numpad1,
    /// <kbd>2 ↓</kbd> on a keyboard. <kbd>2 ABC</kbd> on a phone or remote control
    Numpad2,
    /// <kbd>3 PgDn</kbd> on a keyboard. <kbd>3 DEF</kbd> on a phone or remote control
    Numpad3,
    /// <kbd>4 ←</kbd> on a keyboard. <kbd>4 GHI</kbd> on a phone or remote control
    Numpad4,
    /// <kbd>5</kbd> on a keyboard. <kbd>5 JKL</kbd> on a phone or remote control
    Numpad5,
    /// <kbd>6 →</kbd> on a keyboard. <kbd>6 MNO</kbd> on a phone or remote control
    Numpad6,
    /// <kbd>7 Home</kbd> on a keyboard. <kbd>7 PQRS</kbd> or <kbd>7 PRS</kbd> on a phone
    /// or remote control
    Numpad7,
    /// <kbd>8 ↑</kbd> on a keyboard. <kbd>8 TUV</kbd> on a phone or remote control
    Numpad8,
    /// <kbd>9 PgUp</kbd> on a keyboard. <kbd>9 WXYZ</kbd> or <kbd>9 WXY</kbd> on a phone
    /// or remote control
    Numpad9,
    /// <kbd>+</kbd>
    NumpadAdd,
    /// Found on the Microsoft Natural Keyboard.
    NumpadBackspace,
    /// <kbd>C</kbd> or <kbd>A</kbd> (All Clear). Also for use with numpads that have a
    /// <kbd>Clear</kbd> key that is separate from the <kbd>NumLock</kbd> key. On the Mac, the
    /// numpad <kbd>Clear</kbd> key is encoded as [`NumLock`].
    ///
    /// [`NumLock`]: Self::NumLock
    NumpadClear,
    /// <kbd>C</kbd> (Clear Entry)
    NumpadClearEntry,
    /// <kbd>,</kbd> (thousands separator). For locales where the thousands separator
    /// is a "." (e.g., Brazil), this key may generate a <kbd>.</kbd>.
    NumpadComma,
    /// <kbd>. Del</kbd>. For locales where the decimal separator is "," (e.g.,
    /// Brazil), this key may generate a <kbd>,</kbd>.
    NumpadDecimal,
    /// <kbd>/</kbd>
    NumpadDivide,
    NumpadEnter,
    /// <kbd>=</kbd>
    NumpadEqual,
    /// <kbd>#</kbd> on a phone or remote control device. This key is typically found
    /// below the <kbd>9</kbd> key and to the right of the <kbd>0</kbd> key.
    NumpadHash,
    /// <kbd>M</kbd> Add current entry to the value stored in memory.
    NumpadMemoryAdd,
    /// <kbd>M</kbd> Clear the value stored in memory.
    NumpadMemoryClear,
    /// <kbd>M</kbd> Replace the current entry with the value stored in memory.
    NumpadMemoryRecall,
    /// <kbd>M</kbd> Replace the value stored in memory with the current entry.
    NumpadMemoryStore,
    /// <kbd>M</kbd> Subtract current entry from the value stored in memory.
    NumpadMemorySubtract,
    /// <kbd>*</kbd> on a keyboard. For use with numpads that provide mathematical
    /// operations (<kbd>+</kbd>, <kbd>-</kbd> <kbd>*</kbd> and <kbd>/</kbd>).
    ///
    /// Use `NumpadStar` for the <kbd>*</kbd> key on phones and remote controls.
    NumpadMultiply,
    /// <kbd>(</kbd> Found on the Microsoft Natural Keyboard.
    NumpadParenLeft,
    /// <kbd>)</kbd> Found on the Microsoft Natural Keyboard.
    NumpadParenRight,
    /// <kbd>*</kbd> on a phone or remote control device.
    ///
    /// This key is typically found below the <kbd>7</kbd> key and to the left of
    /// the <kbd>0</kbd> key.
    ///
    /// Use <kbd>"NumpadMultiply"</kbd> for the <kbd>*</kbd> key on
    /// numeric keypads.
    NumpadStar,
    /// <kbd>-</kbd>
    NumpadSubtract,
    /// <kbd>Esc</kbd> or <kbd>⎋</kbd>
    Escape,
    /// <kbd>Fn</kbd> This is typically a hardware key that does not generate a separate code.
    Fn,
    /// <kbd>FLock</kbd> or <kbd>FnLock</kbd>. Function Lock key. Found on the Microsoft
    /// Natural Keyboard.
    FnLock,
    /// <kbd>PrtScr SysRq</kbd> or <kbd>Print Screen</kbd>
    PrintScreen,
    /// <kbd>Scroll Lock</kbd>
    ScrollLock,
    /// <kbd>Pause Break</kbd>
    Pause,
    /// Some laptops place this key to the left of the <kbd>↑</kbd> key.
    ///
    /// This also the "back" button (triangle) on Android.
    BrowserBack,
    BrowserFavorites,
    /// Some laptops place this key to the right of the <kbd>↑</kbd> key.
    BrowserForward,
    /// The "home" button on Android.
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    /// <kbd>Eject</kbd> or <kbd>⏏</kbd>. This key is placed in the function section on some Apple
    /// keyboards.
    Eject,
    /// Sometimes labelled <kbd>My Computer</kbd> on the keyboard
    LaunchApp1,
    /// Sometimes labelled <kbd>Calculator</kbd> on the keyboard
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    /// This key is placed in the function section on some Apple keyboards, replacing the
    /// <kbd>Eject</kbd> key.
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    // Legacy modifier key. Also called "Super" in certain places.
    Meta,
    // Legacy modifier key.
    Hyper,
    Turbo,
    Abort,
    Resume,
    Suspend,
    /// Found on Sun’s USB keyboard.
    Again,
    /// Found on Sun’s USB keyboard.
    Copy,
    /// Found on Sun’s USB keyboard.
    Cut,
    /// Found on Sun’s USB keyboard.
    Find,
    /// Found on Sun’s USB keyboard.
    Open,
    /// Found on Sun’s USB keyboard.
    Paste,
    /// Found on Sun’s USB keyboard.
    Props,
    /// Found on Sun’s USB keyboard.
    Select,
    /// Found on Sun’s USB keyboard.
    Undo,
    /// Use for dedicated <kbd>ひらがな</kbd> key found on some Japanese word processing keyboards.
    Hiragana,
    /// Use for dedicated <kbd>カタカナ</kbd> key found on some Japanese word processing keyboards.
    Katakana,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F1,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F2,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F3,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F4,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F5,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F6,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F7,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F8,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F9,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F10,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F11,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F12,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F13,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F14,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F15,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F16,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F17,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F18,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F19,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F20,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F21,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F22,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F23,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F24,
    /// General-purpose function key.
    F25,
    /// General-purpose function key.
    F26,
    /// General-purpose function key.
    F27,
    /// General-purpose function key.
    F28,
    /// General-purpose function key.
    F29,
    /// General-purpose function key.
    F30,
    /// General-purpose function key.
    F31,
    /// General-purpose function key.
    F32,
    /// General-purpose function key.
    F33,
    /// General-purpose function key.
    F34,
    /// General-purpose function key.
    F35,
}

/// Contains the platform-native physical key identifier
///
/// The exact values vary from platform to platform (which is part of why this is a per-platform
/// enum), but the values are primarily tied to the key's physical location on the keyboard.
///
/// This enum is primarily used to store raw keycodes when Winit doesn't map a given native
/// physical key identifier to a meaningful [`KeyCode`] variant. In the presence of identifiers we
/// haven't mapped for you yet, this lets you use use [`KeyCode`] to:
///
/// - Correctly match key press and release events.
/// - On non-web platforms, support assigning keybinds to virtually any key through a UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[lite_api]
pub enum NativeKeyCode {
    Unidentified,
    /// An Android "scancode".
    Android(i32),
    /// A macOS "scancode".
    MacOS(i32),
    /// A Windows "scancode".
    Windows(i32),
    /// An XKB "keycode".
    Xkb(i32),
}

/// Describes a keyboard input targeting a window.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[lite_api]
pub struct KeyEvent {
    /// Represents the position of a key independent of the currently active layout.
    ///
    /// It also uniquely identifies the physical key (i.e. it's mostly synonymous with a scancode).
    /// The most prevalent use case for this is games. For example the default keys for the player
    /// to move around might be the W, A, S, and D keys on a US layout. The position of these keys
    /// is more important than their label, so they should map to Z, Q, S, and D on an "AZERTY"
    /// layout. (This value is `KeyCode::KeyW` for the Z key on an AZERTY layout.)
    ///
    /// ## Caveats
    ///
    /// - Certain niche hardware will shuffle around physical key positions, e.g. a keyboard that
    /// implements DVORAK in hardware (or firmware)
    /// - Your application will likely have to handle keyboards which are missing keys that your
    /// own keyboard has.
    /// - Certain `KeyCode`s will move between a couple of different positions depending on what
    /// layout the keyboard was manufactured to support.
    ///
    ///  **Because of these caveats, it is important that you provide users with a way to configure
    ///  most (if not all) keybinds in your application.**
    ///
    /// ## `Fn` and `FnLock`
    ///
    /// `Fn` and `FnLock` key events are *exceedingly unlikely* to be emitted by Winit. These keys
    /// are usually handled at the hardware or OS level, and aren't surfaced to applications. If
    /// you somehow see this in the wild, we'd like to know :)
    pub physical_key: PhysicalKey,

    /// Whether the key is being pressed or released.
    ///
    /// See the [`ElementState`] type for more details.
    pub state: ElementState,

    /// Whether or not this key is a key repeat event.
    ///
    /// On some systems, holding down a key for some period of time causes that key to be repeated
    /// as though it were being pressed and released repeatedly. This field is `true` if and only if
    /// this event is the result of one of those repeats.
    pub repeat: bool,
}

/// The location of the key on the keyboard.
///
/// Certain physical keys on the keyboard can have the same value, but are in different locations.
/// For instance, the Shift key can be on the left or right side of the keyboard, or the number
/// keys can be above the letters or on the numpad. This enum allows the user to differentiate
/// them.
///
/// See the documentation for the [`location`] field on the [`KeyEvent`] struct for more information.
///
/// [`location`]: ../event/struct.KeyEvent.html#structfield.location
/// [`KeyEvent`]: crate::event::KeyEvent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[lite_api]
pub enum KeyLocation {
    /// The key is in its "normal" location on the keyboard.
    ///
    /// For instance, the "1" key above the "Q" key on a QWERTY keyboard will use this location. This
    /// invariant is also returned when the location of the key cannot be identified.
    ///
    /// ![Standard 1 key](https://raw.githubusercontent.com/rust-windowing/winit/master/docs/res/keyboard_standard_1_key.svg)
    ///
    /// <sub>
    ///   For image attribution, see the
    ///   <a href="https://github.com/rust-windowing/winit/blob/master/docs/res/ATTRIBUTION.md">
    ///     ATTRIBUTION.md
    ///   </a>
    ///   file.
    /// </sub>
    Standard,

    /// The key is on the left side of the keyboard.
    ///
    /// For instance, the left Shift key below the Caps Lock key on a QWERTY keyboard will use this
    /// location.
    ///
    /// ![Left Shift key](https://raw.githubusercontent.com/rust-windowing/winit/master/docs/res/keyboard_left_shift_key.svg)
    ///
    /// <sub>
    ///   For image attribution, see the
    ///   <a href="https://github.com/rust-windowing/winit/blob/master/docs/res/ATTRIBUTION.md">
    ///     ATTRIBUTION.md
    ///   </a>
    ///   file.
    /// </sub>
    Left,

    /// The key is on the right side of the keyboard.
    ///
    /// For instance, the right Shift key below the Enter key on a QWERTY keyboard will use this
    /// location.
    ///
    /// ![Right Shift key](https://raw.githubusercontent.com/rust-windowing/winit/master/docs/res/keyboard_right_shift_key.svg)
    ///
    /// <sub>
    ///   For image attribution, see the
    ///   <a href="https://github.com/rust-windowing/winit/blob/master/docs/res/ATTRIBUTION.md">
    ///     ATTRIBUTION.md
    ///   </a>
    ///   file.
    /// </sub>
    Right,

    /// The key is on the numpad.
    ///
    /// For instance, the "1" key on the numpad will use this location.
    ///
    /// ![Numpad 1 key](https://raw.githubusercontent.com/rust-windowing/winit/master/docs/res/keyboard_numpad_1_key.svg)
    ///
    /// <sub>
    ///   For image attribution, see the
    ///   <a href="https://github.com/rust-windowing/winit/blob/master/docs/res/ATTRIBUTION.md">
    ///     ATTRIBUTION.md
    ///   </a>
    ///   file.
    /// </sub>
    Numpad,
}

/// Describes touch-screen input state.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[lite_api]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

/// Represents a touch event
///
/// Every time the user touches the screen, a new [`TouchPhase::Started`] event with an unique
/// identifier for the finger is generated. When the finger is lifted, an [`TouchPhase::Ended`]
/// event is generated with the same finger id.
///
/// After a `Started` event has been emitted, there may be zero or more `Move`
/// events when the finger is moved or the touch pressure changes.
///
/// The finger id may be reused by the system after an `Ended` event. The user
/// should assume that a new `Started` event received with the same id has nothing
/// to do with the old finger and is a new finger.
///
/// A [`TouchPhase::Cancelled`] event is emitted when the system has canceled tracking this
/// touch, such as when the window loses focus, or on iOS if the user moves the
/// device against their face.
///
/// ## Platform-specific
///
/// - **Web:** Doesn't take into account CSS [`border`], [`padding`], or [`transform`].
/// - **macOS:** Unsupported.
///
/// [`border`]: https://developer.mozilla.org/en-US/docs/Web/CSS/border
/// [`padding`]: https://developer.mozilla.org/en-US/docs/Web/CSS/padding
/// [`transform`]: https://developer.mozilla.org/en-US/docs/Web/CSS/transform
#[derive(Debug, Clone, Copy, PartialEq)]
#[lite_api]
pub struct Touch {
    pub phase: TouchPhase,
    pub location: PodVector2,
    /// Describes how hard the screen was pressed. May be `None` if the platform
    /// does not support pressure sensitivity.
    ///
    /// ## Platform-specific
    ///
    /// - Only available on **iOS** 9.0+, **Windows** 8+, **Web**, and **Android**.
    /// - **Android**: This will never be [None]. If the device doesn't support pressure
    /// sensitivity, force will either be 0.0 or 1.0. Also see the
    /// [android documentation](https://developer.android.com/reference/android/view/MotionEvent#AXIS_PRESSURE).
    pub force: Option<Force>,
    /// Unique identifier of a finger.
    pub id: i64,
}

/// Describes the force of a touch event
#[derive(Debug, Clone, Copy, PartialEq)]
#[lite_api]
pub enum Force {
    /// On iOS, the force is calibrated so that the same number corresponds to
    /// roughly the same amount of pressure on the screen regardless of the
    /// device.
    Calibrated {
        /// The force of the touch, where a value of 1.0 represents the force of
        /// an average touch (predetermined by the system, not user-specific).
        ///
        /// The force reported by Apple Pencil is measured along the axis of the
        /// pencil. If you want a force perpendicular to the device, you need to
        /// calculate this value using the `altitude_angle` value.
        force: f64,
        /// The maximum possible force for a touch.
        ///
        /// The value of this field is sufficiently high to provide a wide
        /// dynamic range for values of the `force` field.
        max_possible_force: f64,
        /// The altitude (in radians) of the stylus.
        ///
        /// A value of 0 radians indicates that the stylus is parallel to the
        /// surface. The value of this property is Pi/2 when the stylus is
        /// perpendicular to the surface.
        altitude_angle: Option<f64>,
    },
    /// If the platform reports the force as normalized, we have no way of
    /// knowing how much pressure 1.0 corresponds to – we know it's the maximum
    /// amount of force, but as to how much force, you might either have to
    /// press really really hard, or not hard at all, depending on the device.
    Normalized(f64),
}

/// Identifier for a specific analog axis on some device.
// pub type AxisId = i32;

/// Identifier for a specific button on some device.
// pub type ButtonId = u32;

/// Describes the input state of a key.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[lite_api]
pub enum ElementState {
    Pressed,
    Released,
}

/// Describes a button of a mouse controller.
///
/// ## Platform-specific
///
/// **macOS:** `Back` and `Forward` might not work with all hardware.
/// **Orbital:** `Back` and `Forward` are unsupported due to orbital not supporting them.
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[lite_api]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(i32),
}

/// Describes a difference in the mouse scroll wheel state.
#[derive(Debug, Clone, Copy, PartialEq)]
#[lite_api]
pub enum MouseScrollDelta {
    /// Amount in lines or rows to scroll in the horizontal
    /// and vertical directions.
    ///
    /// Positive values indicate that the content that is being scrolled should move
    /// right and down (revealing more content left and up).
    LineDelta(PodVector2),

    /// Amount in pixels to scroll in the horizontal and
    /// vertical direction.
    ///
    /// Scroll events are expressed as a `PixelDelta` if
    /// supported by the device (eg. a touchpad) and
    /// platform.
    ///
    /// Positive values indicate that the content being scrolled should
    /// move right/down.
    ///
    /// For a 'natural scrolling' touch pad (that acts like a touch screen)
    /// this means moving your fingers right and down should give positive values,
    /// and move the content right and down (to reveal more things left and up).
    PixelDelta(PodVector2),
}

/// Handle to synchroniously change the size of the window from the
/// [`WindowEvent`].
#[derive(Debug, Clone)]
pub struct InnerSizeWriter {
    pub(crate) target: fyrox::event::InnerSizeWriter,
}

#[lite_api(class=InnerSizeWriter)]
impl InnerSizeWriter {
    /// Try to request inner size which will be set synchroniously on the window.
    pub fn request_inner_size(&mut self, new_inner_size: PodVector2i) -> bool {
        self.target
            .request_inner_size(PhysicalSize::new(
                new_inner_size.x as u32,
                new_inner_size.y as u32,
            ))
            .is_ok()
    }
}

impl PartialEq for InnerSizeWriter {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target
    }
}

#[allow(unused_variables)]
pub fn to_lite<T>(event: fyrox::event::Event<T>) -> Option<Event> {
    Some(match event {
        fyrox::event::Event::NewEvents(it) => return None,
        fyrox::event::Event::WindowEvent { window_id, event } => Event::WindowEvent {
            window_id: u64::from(window_id) as i64,
            event: windo_event_to_lite(event)?,
        },
        fyrox::event::Event::DeviceEvent { device_id, event } => Event::DeviceEvent {
            event: match event {
                fyrox::event::DeviceEvent::Added => DeviceEvent::Added,
                fyrox::event::DeviceEvent::Removed => DeviceEvent::Removed,
                fyrox::event::DeviceEvent::MouseMotion { delta: (x, y) } => {
                    DeviceEvent::MouseMotion {
                        delta: PodVector2 {
                            x: x as f32,
                            y: y as f32,
                        },
                    }
                }
                fyrox::event::DeviceEvent::MouseWheel { delta } => DeviceEvent::MouseWheel {
                    delta: delta.into(),
                },
                fyrox::event::DeviceEvent::Motion { axis, value } => DeviceEvent::Motion {
                    axis: axis as i32,
                    value,
                },
                fyrox::event::DeviceEvent::Button { button, state } => DeviceEvent::Button {
                    button: button as i32,
                    state: state.into(),
                },
                fyrox::event::DeviceEvent::Key(it) => DeviceEvent::Key(RawKeyEvent {
                    physical_key: physical_key_to_lite(it.physical_key)?,
                    state: it.state.into(),
                }),
            },
        },
        fyrox::event::Event::UserEvent(_) => return None,
        fyrox::event::Event::Suspended => Event::Suspended,
        fyrox::event::Event::Resumed => Event::Resumed,
        fyrox::event::Event::AboutToWait => Event::AboutToWait,
        fyrox::event::Event::LoopExiting => Event::LoopExiting,
        fyrox::event::Event::MemoryWarning => Event::MemoryWarning,
    })
}

#[allow(unused_variables)]
pub fn windo_event_to_lite(event: fyrox::event::WindowEvent) -> Option<WindowEvent> {
    Some(match event {
        fyrox::event::WindowEvent::ActivationTokenDone { serial, token } => {
            WindowEvent::ActivationTokenDone
        }
        fyrox::event::WindowEvent::Resized(PhysicalSize { width, height }) => {
            WindowEvent::Resized(PodVector2i {
                x: width as i64,
                y: height as i64,
            })
        }
        fyrox::event::WindowEvent::Moved(PhysicalPosition { x, y }) => {
            WindowEvent::Moved(PodVector2i {
                x: x as i64,
                y: y as i64,
            })
        }
        fyrox::event::WindowEvent::CloseRequested => WindowEvent::CloseRequested,
        fyrox::event::WindowEvent::Destroyed => WindowEvent::Destroyed,
        fyrox::event::WindowEvent::DroppedFile(path_buf) => {
            WindowEvent::DroppedFile(path_buf.to_string_lossy().to_string())
        }
        fyrox::event::WindowEvent::HoveredFile(path_buf) => {
            WindowEvent::HoveredFile(path_buf.to_string_lossy().to_string())
        }
        fyrox::event::WindowEvent::HoveredFileCancelled => WindowEvent::HoveredFileCancelled,
        fyrox::event::WindowEvent::Focused(v) => WindowEvent::Focused(v),
        fyrox::event::WindowEvent::KeyboardInput {
            device_id,
            event,
            is_synthetic,
        } => WindowEvent::KeyboardInput {
            event: KeyEvent {
                physical_key: physical_key_to_lite(event.physical_key)?,
                state: event.state.into(),
                repeat: event.repeat,
            },
            is_synthetic,
        },
        fyrox::event::WindowEvent::ModifiersChanged(modifiers) => WindowEvent::ModifiersChanged,
        fyrox::event::WindowEvent::Ime(ime) => WindowEvent::Ime,
        fyrox::event::WindowEvent::CursorMoved {
            device_id,
            position,
        } => WindowEvent::CursorMoved {
            position: PodVector2i {
                x: position.x as i64,
                y: position.y as i64,
            },
        },
        fyrox::event::WindowEvent::CursorEntered { device_id } => WindowEvent::CursorEntered,
        fyrox::event::WindowEvent::CursorLeft { device_id } => WindowEvent::CursorLeft,
        fyrox::event::WindowEvent::MouseWheel {
            device_id,
            delta,
            phase,
        } => WindowEvent::MouseWheel {
            delta: delta.into(),
            phase: phase.into(),
        },
        fyrox::event::WindowEvent::MouseInput {
            device_id,
            state,
            button,
        } => WindowEvent::MouseInput {
            state: state.into(),
            button: match button {
                fyrox::event::MouseButton::Left => MouseButton::Left,
                fyrox::event::MouseButton::Right => MouseButton::Right,
                fyrox::event::MouseButton::Middle => MouseButton::Middle,
                fyrox::event::MouseButton::Back => MouseButton::Back,
                fyrox::event::MouseButton::Forward => MouseButton::Forward,
                fyrox::event::MouseButton::Other(it) => MouseButton::Other(it as i32),
            },
        },
        fyrox::event::WindowEvent::TouchpadMagnify {
            device_id,
            delta,
            phase,
        } => WindowEvent::TouchpadMagnify {
            delta,
            phase: phase.into(),
        },
        fyrox::event::WindowEvent::SmartMagnify { device_id } => WindowEvent::SmartMagnify,
        fyrox::event::WindowEvent::TouchpadRotate {
            device_id,
            delta,
            phase,
        } => WindowEvent::TouchpadRotate {
            delta,
            phase: phase.into(),
        },
        fyrox::event::WindowEvent::TouchpadPressure {
            device_id,
            pressure,
            stage,
        } => WindowEvent::TouchpadPressure { pressure, stage },
        fyrox::event::WindowEvent::AxisMotion {
            device_id,
            axis,
            value,
        } => WindowEvent::AxisMotion {
            axis: axis as i32,
            value,
        },
        fyrox::event::WindowEvent::Touch(fyrox::event::Touch {
            device_id,
            phase,
            location: PhysicalPosition { x, y },
            force,
            id,
        }) => WindowEvent::Touch(Touch {
            phase: phase.into(),
            location: PodVector2 {
                x: x as f32,
                y: y as f32,
            },
            force: force.map(|it| match it {
                fyrox::event::Force::Calibrated {
                    force,
                    max_possible_force,
                    altitude_angle,
                } => Force::Calibrated {
                    force,
                    max_possible_force,
                    altitude_angle,
                },
                fyrox::event::Force::Normalized(it) => Force::Normalized(it),
            }),
            id: id as i64,
        }),
        fyrox::event::WindowEvent::ScaleFactorChanged {
            scale_factor,
            inner_size_writer,
        } => WindowEvent::ScaleFactorChanged {
            scale_factor,
            inner_size_writer: InnerSizeWriter {
                target: inner_size_writer,
            },
        },
        fyrox::event::WindowEvent::ThemeChanged(theme) => WindowEvent::ThemeChanged,
        fyrox::event::WindowEvent::Occluded(it) => WindowEvent::Occluded(it),
        fyrox::event::WindowEvent::RedrawRequested => WindowEvent::RedrawRequested,
    })
}

impl From<fyrox::event::MouseScrollDelta> for MouseScrollDelta {
    fn from(value: fyrox::event::MouseScrollDelta) -> Self {
        match value {
            fyrox::event::MouseScrollDelta::LineDelta(x, y) => {
                MouseScrollDelta::LineDelta(PodVector2 { x, y })
            }
            fyrox::event::MouseScrollDelta::PixelDelta(PhysicalPosition { x, y }) => {
                MouseScrollDelta::PixelDelta(PodVector2 {
                    x: x as f32,
                    y: y as f32,
                })
            }
        }
    }
}

impl From<fyrox::event::ElementState> for ElementState {
    fn from(value: fyrox::event::ElementState) -> Self {
        match value {
            fyrox::event::ElementState::Pressed => ElementState::Pressed,
            fyrox::event::ElementState::Released => ElementState::Released,
        }
    }
}

impl From<fyrox::event::TouchPhase> for TouchPhase {
    fn from(value: fyrox::event::TouchPhase) -> Self {
        match value {
            fyrox::event::TouchPhase::Started => TouchPhase::Started,
            fyrox::event::TouchPhase::Moved => TouchPhase::Moved,
            fyrox::event::TouchPhase::Ended => TouchPhase::Ended,
            fyrox::event::TouchPhase::Cancelled => TouchPhase::Cancelled,
        }
    }
}

fn physical_key_to_lite(it: fyrox::keyboard::PhysicalKey) -> Option<PhysicalKey> {
    Some(match it {
        fyrox::keyboard::PhysicalKey::Code(it) => PhysicalKey::Code(match it {
            fyrox::keyboard::KeyCode::Backquote => KeyCode::Backquote,
            fyrox::keyboard::KeyCode::Backslash => KeyCode::Backslash,
            fyrox::keyboard::KeyCode::BracketLeft => KeyCode::BracketLeft,
            fyrox::keyboard::KeyCode::BracketRight => KeyCode::BracketRight,
            fyrox::keyboard::KeyCode::Comma => KeyCode::Comma,
            fyrox::keyboard::KeyCode::Digit0 => KeyCode::Digit0,
            fyrox::keyboard::KeyCode::Digit1 => KeyCode::Digit1,
            fyrox::keyboard::KeyCode::Digit2 => KeyCode::Digit2,
            fyrox::keyboard::KeyCode::Digit3 => KeyCode::Digit3,
            fyrox::keyboard::KeyCode::Digit4 => KeyCode::Digit4,
            fyrox::keyboard::KeyCode::Digit5 => KeyCode::Digit5,
            fyrox::keyboard::KeyCode::Digit6 => KeyCode::Digit6,
            fyrox::keyboard::KeyCode::Digit7 => KeyCode::Digit7,
            fyrox::keyboard::KeyCode::Digit8 => KeyCode::Digit8,
            fyrox::keyboard::KeyCode::Digit9 => KeyCode::Digit9,
            fyrox::keyboard::KeyCode::Equal => KeyCode::Equal,
            fyrox::keyboard::KeyCode::IntlBackslash => KeyCode::IntlBackslash,
            fyrox::keyboard::KeyCode::IntlRo => KeyCode::IntlRo,
            fyrox::keyboard::KeyCode::IntlYen => KeyCode::IntlYen,
            fyrox::keyboard::KeyCode::KeyA => KeyCode::KeyA,
            fyrox::keyboard::KeyCode::KeyB => KeyCode::KeyB,
            fyrox::keyboard::KeyCode::KeyC => KeyCode::KeyC,
            fyrox::keyboard::KeyCode::KeyD => KeyCode::KeyD,
            fyrox::keyboard::KeyCode::KeyE => KeyCode::KeyE,
            fyrox::keyboard::KeyCode::KeyF => KeyCode::KeyF,
            fyrox::keyboard::KeyCode::KeyG => KeyCode::KeyG,
            fyrox::keyboard::KeyCode::KeyH => KeyCode::KeyH,
            fyrox::keyboard::KeyCode::KeyI => KeyCode::KeyI,
            fyrox::keyboard::KeyCode::KeyJ => KeyCode::KeyJ,
            fyrox::keyboard::KeyCode::KeyK => KeyCode::KeyK,
            fyrox::keyboard::KeyCode::KeyL => KeyCode::KeyL,
            fyrox::keyboard::KeyCode::KeyM => KeyCode::KeyM,
            fyrox::keyboard::KeyCode::KeyN => KeyCode::KeyN,
            fyrox::keyboard::KeyCode::KeyO => KeyCode::KeyO,
            fyrox::keyboard::KeyCode::KeyP => KeyCode::KeyP,
            fyrox::keyboard::KeyCode::KeyQ => KeyCode::KeyQ,
            fyrox::keyboard::KeyCode::KeyR => KeyCode::KeyR,
            fyrox::keyboard::KeyCode::KeyS => KeyCode::KeyS,
            fyrox::keyboard::KeyCode::KeyT => KeyCode::KeyT,
            fyrox::keyboard::KeyCode::KeyU => KeyCode::KeyU,
            fyrox::keyboard::KeyCode::KeyV => KeyCode::KeyV,
            fyrox::keyboard::KeyCode::KeyW => KeyCode::KeyW,
            fyrox::keyboard::KeyCode::KeyX => KeyCode::KeyX,
            fyrox::keyboard::KeyCode::KeyY => KeyCode::KeyY,
            fyrox::keyboard::KeyCode::KeyZ => KeyCode::KeyZ,
            fyrox::keyboard::KeyCode::Minus => KeyCode::Minus,
            fyrox::keyboard::KeyCode::Period => KeyCode::Period,
            fyrox::keyboard::KeyCode::Quote => KeyCode::Quote,
            fyrox::keyboard::KeyCode::Semicolon => KeyCode::Semicolon,
            fyrox::keyboard::KeyCode::Slash => KeyCode::Slash,
            fyrox::keyboard::KeyCode::AltLeft => KeyCode::AltLeft,
            fyrox::keyboard::KeyCode::AltRight => KeyCode::AltRight,
            fyrox::keyboard::KeyCode::Backspace => KeyCode::Backspace,
            fyrox::keyboard::KeyCode::CapsLock => KeyCode::CapsLock,
            fyrox::keyboard::KeyCode::ContextMenu => KeyCode::ContextMenu,
            fyrox::keyboard::KeyCode::ControlLeft => KeyCode::ControlLeft,
            fyrox::keyboard::KeyCode::ControlRight => KeyCode::ControlRight,
            fyrox::keyboard::KeyCode::Enter => KeyCode::Enter,
            fyrox::keyboard::KeyCode::SuperLeft => KeyCode::SuperLeft,
            fyrox::keyboard::KeyCode::SuperRight => KeyCode::SuperRight,
            fyrox::keyboard::KeyCode::ShiftLeft => KeyCode::ShiftLeft,
            fyrox::keyboard::KeyCode::ShiftRight => KeyCode::ShiftRight,
            fyrox::keyboard::KeyCode::Space => KeyCode::Space,
            fyrox::keyboard::KeyCode::Tab => KeyCode::Tab,
            fyrox::keyboard::KeyCode::Convert => KeyCode::Convert,
            fyrox::keyboard::KeyCode::KanaMode => KeyCode::KanaMode,
            fyrox::keyboard::KeyCode::Lang1 => KeyCode::Lang1,
            fyrox::keyboard::KeyCode::Lang2 => KeyCode::Lang2,
            fyrox::keyboard::KeyCode::Lang3 => KeyCode::Lang3,
            fyrox::keyboard::KeyCode::Lang4 => KeyCode::Lang4,
            fyrox::keyboard::KeyCode::Lang5 => KeyCode::Lang5,
            fyrox::keyboard::KeyCode::NonConvert => KeyCode::NonConvert,
            fyrox::keyboard::KeyCode::Delete => KeyCode::Delete,
            fyrox::keyboard::KeyCode::End => KeyCode::End,
            fyrox::keyboard::KeyCode::Help => KeyCode::Help,
            fyrox::keyboard::KeyCode::Home => KeyCode::Home,
            fyrox::keyboard::KeyCode::Insert => KeyCode::Insert,
            fyrox::keyboard::KeyCode::PageDown => KeyCode::PageDown,
            fyrox::keyboard::KeyCode::PageUp => KeyCode::PageUp,
            fyrox::keyboard::KeyCode::ArrowDown => KeyCode::ArrowDown,
            fyrox::keyboard::KeyCode::ArrowLeft => KeyCode::ArrowLeft,
            fyrox::keyboard::KeyCode::ArrowRight => KeyCode::ArrowRight,
            fyrox::keyboard::KeyCode::ArrowUp => KeyCode::ArrowUp,
            fyrox::keyboard::KeyCode::NumLock => KeyCode::NumLock,
            fyrox::keyboard::KeyCode::Numpad0 => KeyCode::Numpad0,
            fyrox::keyboard::KeyCode::Numpad1 => KeyCode::Numpad1,
            fyrox::keyboard::KeyCode::Numpad2 => KeyCode::Numpad2,
            fyrox::keyboard::KeyCode::Numpad3 => KeyCode::Numpad3,
            fyrox::keyboard::KeyCode::Numpad4 => KeyCode::Numpad4,
            fyrox::keyboard::KeyCode::Numpad5 => KeyCode::Numpad5,
            fyrox::keyboard::KeyCode::Numpad6 => KeyCode::Numpad6,
            fyrox::keyboard::KeyCode::Numpad7 => KeyCode::Numpad7,
            fyrox::keyboard::KeyCode::Numpad8 => KeyCode::Numpad8,
            fyrox::keyboard::KeyCode::Numpad9 => KeyCode::Numpad9,
            fyrox::keyboard::KeyCode::NumpadAdd => KeyCode::NumpadAdd,
            fyrox::keyboard::KeyCode::NumpadBackspace => KeyCode::NumpadBackspace,
            fyrox::keyboard::KeyCode::NumpadClear => KeyCode::NumpadClear,
            fyrox::keyboard::KeyCode::NumpadClearEntry => KeyCode::NumpadClearEntry,
            fyrox::keyboard::KeyCode::NumpadComma => KeyCode::NumpadComma,
            fyrox::keyboard::KeyCode::NumpadDecimal => KeyCode::NumpadDecimal,
            fyrox::keyboard::KeyCode::NumpadDivide => KeyCode::NumpadDivide,
            fyrox::keyboard::KeyCode::NumpadEnter => KeyCode::NumpadEnter,
            fyrox::keyboard::KeyCode::NumpadEqual => KeyCode::NumpadEqual,
            fyrox::keyboard::KeyCode::NumpadHash => KeyCode::NumpadHash,
            fyrox::keyboard::KeyCode::NumpadMemoryAdd => KeyCode::NumpadMemoryAdd,
            fyrox::keyboard::KeyCode::NumpadMemoryClear => KeyCode::NumpadMemoryClear,
            fyrox::keyboard::KeyCode::NumpadMemoryRecall => KeyCode::NumpadMemoryRecall,
            fyrox::keyboard::KeyCode::NumpadMemoryStore => KeyCode::NumpadMemoryStore,
            fyrox::keyboard::KeyCode::NumpadMemorySubtract => KeyCode::NumpadMemorySubtract,
            fyrox::keyboard::KeyCode::NumpadMultiply => KeyCode::NumpadMultiply,
            fyrox::keyboard::KeyCode::NumpadParenLeft => KeyCode::NumpadParenLeft,
            fyrox::keyboard::KeyCode::NumpadParenRight => KeyCode::NumpadParenRight,
            fyrox::keyboard::KeyCode::NumpadStar => KeyCode::NumpadStar,
            fyrox::keyboard::KeyCode::NumpadSubtract => KeyCode::NumpadSubtract,
            fyrox::keyboard::KeyCode::Escape => KeyCode::Escape,
            fyrox::keyboard::KeyCode::Fn => KeyCode::Fn,
            fyrox::keyboard::KeyCode::FnLock => KeyCode::FnLock,
            fyrox::keyboard::KeyCode::PrintScreen => KeyCode::PrintScreen,
            fyrox::keyboard::KeyCode::ScrollLock => KeyCode::ScrollLock,
            fyrox::keyboard::KeyCode::Pause => KeyCode::Pause,
            fyrox::keyboard::KeyCode::BrowserBack => KeyCode::BrowserBack,
            fyrox::keyboard::KeyCode::BrowserFavorites => KeyCode::BrowserFavorites,
            fyrox::keyboard::KeyCode::BrowserForward => KeyCode::BrowserForward,
            fyrox::keyboard::KeyCode::BrowserHome => KeyCode::BrowserHome,
            fyrox::keyboard::KeyCode::BrowserRefresh => KeyCode::BrowserRefresh,
            fyrox::keyboard::KeyCode::BrowserSearch => KeyCode::BrowserSearch,
            fyrox::keyboard::KeyCode::BrowserStop => KeyCode::BrowserStop,
            fyrox::keyboard::KeyCode::Eject => KeyCode::Eject,
            fyrox::keyboard::KeyCode::LaunchApp1 => KeyCode::LaunchApp1,
            fyrox::keyboard::KeyCode::LaunchApp2 => KeyCode::LaunchApp2,
            fyrox::keyboard::KeyCode::LaunchMail => KeyCode::LaunchMail,
            fyrox::keyboard::KeyCode::MediaPlayPause => KeyCode::MediaPlayPause,
            fyrox::keyboard::KeyCode::MediaSelect => KeyCode::MediaSelect,
            fyrox::keyboard::KeyCode::MediaStop => KeyCode::MediaStop,
            fyrox::keyboard::KeyCode::MediaTrackNext => KeyCode::MediaTrackNext,
            fyrox::keyboard::KeyCode::MediaTrackPrevious => KeyCode::MediaTrackPrevious,
            fyrox::keyboard::KeyCode::Power => KeyCode::Power,
            fyrox::keyboard::KeyCode::Sleep => KeyCode::Sleep,
            fyrox::keyboard::KeyCode::AudioVolumeDown => KeyCode::AudioVolumeDown,
            fyrox::keyboard::KeyCode::AudioVolumeMute => KeyCode::AudioVolumeMute,
            fyrox::keyboard::KeyCode::AudioVolumeUp => KeyCode::AudioVolumeUp,
            fyrox::keyboard::KeyCode::WakeUp => KeyCode::WakeUp,
            fyrox::keyboard::KeyCode::Meta => KeyCode::Meta,
            fyrox::keyboard::KeyCode::Hyper => KeyCode::Hyper,
            fyrox::keyboard::KeyCode::Turbo => KeyCode::Turbo,
            fyrox::keyboard::KeyCode::Abort => KeyCode::Abort,
            fyrox::keyboard::KeyCode::Resume => KeyCode::Resume,
            fyrox::keyboard::KeyCode::Suspend => KeyCode::Suspend,
            fyrox::keyboard::KeyCode::Again => KeyCode::Again,
            fyrox::keyboard::KeyCode::Copy => KeyCode::Copy,
            fyrox::keyboard::KeyCode::Cut => KeyCode::Cut,
            fyrox::keyboard::KeyCode::Find => KeyCode::Find,
            fyrox::keyboard::KeyCode::Open => KeyCode::Open,
            fyrox::keyboard::KeyCode::Paste => KeyCode::Paste,
            fyrox::keyboard::KeyCode::Props => KeyCode::Props,
            fyrox::keyboard::KeyCode::Select => KeyCode::Select,
            fyrox::keyboard::KeyCode::Undo => KeyCode::Undo,
            fyrox::keyboard::KeyCode::Hiragana => KeyCode::Hiragana,
            fyrox::keyboard::KeyCode::Katakana => KeyCode::Katakana,
            fyrox::keyboard::KeyCode::F1 => KeyCode::F1,
            fyrox::keyboard::KeyCode::F2 => KeyCode::F2,
            fyrox::keyboard::KeyCode::F3 => KeyCode::F3,
            fyrox::keyboard::KeyCode::F4 => KeyCode::F4,
            fyrox::keyboard::KeyCode::F5 => KeyCode::F5,
            fyrox::keyboard::KeyCode::F6 => KeyCode::F6,
            fyrox::keyboard::KeyCode::F7 => KeyCode::F7,
            fyrox::keyboard::KeyCode::F8 => KeyCode::F8,
            fyrox::keyboard::KeyCode::F9 => KeyCode::F9,
            fyrox::keyboard::KeyCode::F10 => KeyCode::F10,
            fyrox::keyboard::KeyCode::F11 => KeyCode::F11,
            fyrox::keyboard::KeyCode::F12 => KeyCode::F12,
            fyrox::keyboard::KeyCode::F13 => KeyCode::F13,
            fyrox::keyboard::KeyCode::F14 => KeyCode::F14,
            fyrox::keyboard::KeyCode::F15 => KeyCode::F15,
            fyrox::keyboard::KeyCode::F16 => KeyCode::F16,
            fyrox::keyboard::KeyCode::F17 => KeyCode::F17,
            fyrox::keyboard::KeyCode::F18 => KeyCode::F18,
            fyrox::keyboard::KeyCode::F19 => KeyCode::F19,
            fyrox::keyboard::KeyCode::F20 => KeyCode::F20,
            fyrox::keyboard::KeyCode::F21 => KeyCode::F21,
            fyrox::keyboard::KeyCode::F22 => KeyCode::F22,
            fyrox::keyboard::KeyCode::F23 => KeyCode::F23,
            fyrox::keyboard::KeyCode::F24 => KeyCode::F24,
            fyrox::keyboard::KeyCode::F25 => KeyCode::F25,
            fyrox::keyboard::KeyCode::F26 => KeyCode::F26,
            fyrox::keyboard::KeyCode::F27 => KeyCode::F27,
            fyrox::keyboard::KeyCode::F28 => KeyCode::F28,
            fyrox::keyboard::KeyCode::F29 => KeyCode::F29,
            fyrox::keyboard::KeyCode::F30 => KeyCode::F30,
            fyrox::keyboard::KeyCode::F31 => KeyCode::F31,
            fyrox::keyboard::KeyCode::F32 => KeyCode::F32,
            fyrox::keyboard::KeyCode::F33 => KeyCode::F33,
            fyrox::keyboard::KeyCode::F34 => KeyCode::F34,
            fyrox::keyboard::KeyCode::F35 => KeyCode::F35,
            _ => return None,
        }),
        fyrox::keyboard::PhysicalKey::Unidentified(it) => PhysicalKey::Unidentified(match it {
            fyrox::keyboard::NativeKeyCode::Unidentified => NativeKeyCode::Unidentified,
            fyrox::keyboard::NativeKeyCode::Android(it) => NativeKeyCode::Android(it as i32),
            fyrox::keyboard::NativeKeyCode::MacOS(it) => NativeKeyCode::MacOS(it as i32),
            fyrox::keyboard::NativeKeyCode::Windows(it) => NativeKeyCode::Windows(it as i32),
            fyrox::keyboard::NativeKeyCode::Xkb(it) => NativeKeyCode::Xkb(it as i32),
        }),
    })
}
