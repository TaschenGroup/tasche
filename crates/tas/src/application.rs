use std::{cell::RefCell, error::Error, fmt, rc::Rc};

use gpui_kit::{App, Application, Entity, QuitMode, Render, Window, WindowOptions};

/// Identifies the stage at which application startup failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunErrorKind {
    /// The current platform is not supported by this application entry point.
    UnsupportedPlatform,
    /// The Windows platform could not be initialized.
    PlatformInitialization,
    /// The initial application window could not be opened.
    WindowCreation,
}

/// A platform initialization or initial window creation error.
#[derive(Debug)]
pub struct RunError {
    kind: RunErrorKind,
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl RunError {
    /// Returns the stage at which application startup failed.
    pub fn kind(&self) -> RunErrorKind {
        self.kind
    }
}

impl fmt::Display for RunError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stage = match self.kind {
            RunErrorKind::UnsupportedPlatform => "unsupported platform",
            RunErrorKind::PlatformInitialization => "failed to initialize the Windows platform",
            RunErrorKind::WindowCreation => "failed to open the application window",
        };
        write!(formatter, "{stage}: {}", self.message)
    }
}

impl Error for RunError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn Error + 'static))
    }
}

/// Runs a single-window application using the GPUI Kit window root.
///
/// The builder creates the window content on the GPUI thread. The event loop
/// runs until the last window closes. Returns an error if platform
/// initialization or initial window creation fails. Call this function on the
/// thread that owns the UI loop. A platform initialization failure terminates
/// the startup attempt; retrying on the same thread is not supported.
///
/// # Panics
///
/// GPUI may panic inside platform initialization or the supplied builder.
pub fn run<V, F>(options: WindowOptions, build: F) -> Result<(), RunError>
where
    V: Render + 'static,
    F: FnOnce(&mut Window, &mut App) -> Entity<V> + 'static,
{
    let error = Rc::new(RefCell::new(None));
    let startup_error = Rc::clone(&error);

    create_application()?
        .with_quit_mode(QuitMode::LastWindowClosed)
        .run(move |cx| {
            gpui_kit::init(cx);

            let result = gpui_kit::open_window(options, cx, build).map(|_| ());
            finish_window_startup(result, &startup_error, || cx.quit());
        });

    match error.borrow_mut().take() {
        Some(err) => Err(err),
        None => Ok(()),
    }
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
fn create_application() -> Result<Application, RunError> {
    let platform = platform_startup_result(gpui_windows::WindowsPlatform::new(false))?;
    Ok(Application::with_platform(Rc::new(platform)))
}

#[cfg(any(all(target_os = "windows", target_arch = "x86_64"), test))]
fn platform_startup_result<T>(result: gpui_kit::Result<T>) -> Result<T, RunError> {
    result.map_err(|err| RunError {
        kind: RunErrorKind::PlatformInitialization,
        message: format!("{err:#}"),
        source: Some(err.into_boxed_dyn_error()),
    })
}

#[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]
fn create_application() -> Result<Application, RunError> {
    Err(RunError {
        kind: RunErrorKind::UnsupportedPlatform,
        message: "Tasche currently supports Windows x64".to_owned(),
        source: None,
    })
}

fn finish_window_startup(
    result: gpui_kit::Result<()>,
    startup_error: &RefCell<Option<RunError>>,
    quit: impl FnOnce(),
) {
    if let Err(err) = result {
        *startup_error.borrow_mut() = Some(RunError {
            kind: RunErrorKind::WindowCreation,
            message: format!("{err:#}"),
            source: Some(err.into_boxed_dyn_error()),
        });
        quit();
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cell::{Cell, RefCell},
        error::Error,
    };

    use super::{RunErrorKind, finish_window_startup, platform_startup_result};

    #[test]
    fn platform_failure_returns_its_stage_and_error_chain() {
        let failure: gpui_kit::Result<()> =
            Err(std::io::Error::other("injected platform failure").into());
        let error =
            platform_startup_result(failure.map_err(|error| error.context("platform backend")))
                .expect_err("platform failure must be reported");

        assert_eq!(error.kind(), RunErrorKind::PlatformInitialization);
        assert!(error.to_string().contains("platform backend"));
        assert!(error.to_string().contains("injected platform failure"));
        let source = error.source().expect("source error must be retained");
        assert!(source.source().is_some());
    }

    #[test]
    fn window_failure_returns_error_and_requests_quit() {
        let quit_count = Cell::new(0);
        let startup_error = RefCell::new(None);
        let failure: gpui_kit::Result<()> =
            Err(std::io::Error::other("injected window failure").into());
        finish_window_startup(
            failure.map_err(|error| error.context("window backend")),
            &startup_error,
            || {
                assert!(startup_error.borrow().is_some());
                quit_count.set(quit_count.get() + 1);
            },
        );

        let error = startup_error
            .into_inner()
            .expect("window failure must be reported");
        assert_eq!(error.kind(), RunErrorKind::WindowCreation);
        assert!(error.to_string().contains("window backend"));
        assert!(error.to_string().contains("injected window failure"));
        let source = error.source().expect("source error must be retained");
        assert!(source.source().is_some());
        assert_eq!(quit_count.get(), 1);
    }

    #[test]
    fn successful_window_startup_does_not_request_quit() {
        let quit_count = Cell::new(0);
        let startup_error = RefCell::new(None);
        finish_window_startup(Ok(()), &startup_error, || {
            quit_count.set(quit_count.get() + 1);
        });

        assert!(startup_error.into_inner().is_none());
        assert_eq!(quit_count.get(), 0);
    }
}
