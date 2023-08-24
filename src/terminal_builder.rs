#[allow(unused_imports)]
use std::io::{self, stderr, stdout, Stderr, Stdout, Write};

use crate::{
    backend::{self},
    Terminal,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum RawMode {
    #[default]
    Enabled,
    Disabled,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AlternateScreenMode {
    #[default]
    EnterAlternateScreen,
    MainScreen,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MouseCapture {
    Captured,
    #[default]
    NotCaptured,
}

#[derive(Debug)]
pub struct TerminalBuilder;

impl TerminalBuilder {
    #[cfg(feature = "crossterm")]
    pub fn crossterm_on_stdout() -> crossterm::TerminalBuilder<Stdout> {
        Self::crossterm(stdout())
    }

    #[cfg(feature = "crossterm")]
    pub fn crossterm_on_stderr() -> crossterm::TerminalBuilder<Stderr> {
        Self::crossterm(stderr())
    }

    #[cfg(feature = "crossterm")]
    pub fn crossterm<W: Write>(writer: W) -> crossterm::TerminalBuilder<W> {
        crossterm::TerminalBuilder::new(writer)
    }

    #[cfg(feature = "termion")]
    pub fn termion_on_stdout() -> termion::TerminalBuilder<Stdout> {
        Self::termion(stdout())
    }

    #[cfg(feature = "termion")]
    pub fn termion_on_stderr() -> termion::TerminalBuilder<Stderr> {
        Self::termion(stderr())
    }

    #[cfg(feature = "termion")]
    pub fn termion<W: Write>(writer: W) -> termion::TerminalBuilder<W> {
        termion::TerminalBuilder::new(writer)
    }

    #[cfg(feature = "termwiz")]
    pub fn termwiz_on_stdout() -> termwiz::TerminalBuilder {
        termwiz::TerminalBuilder::new()
    }
}

#[cfg(feature = "crossterm")]
mod crossterm {
    use super::*;
    use crate::backend::CrosstermBackend;

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct TerminalBuilder<W: Write> {
        writer: W,
        raw_mode: RawMode,
        alternate_screen: AlternateScreenMode,
        mouse_capture: MouseCapture,
    }

    impl<W: Write> TerminalBuilder<W> {
        pub fn new(writer: W) -> Self {
            TerminalBuilder::<W> {
                writer,
                raw_mode: RawMode::default(),
                alternate_screen: AlternateScreenMode::default(),
                mouse_capture: MouseCapture::default(),
            }
        }

        pub fn raw_mode(self, raw_mode: RawMode) -> Self {
            TerminalBuilder::<W> { raw_mode, ..self }
        }

        pub fn alternate_screen(self, alternate_screen: AlternateScreenMode) -> Self {
            TerminalBuilder::<W> {
                alternate_screen,
                ..self
            }
        }

        pub fn mouse_capture(self, mouse_capture: MouseCapture) -> Self {
            TerminalBuilder::<W> {
                mouse_capture,
                ..self
            }
        }

        pub fn build(self) -> io::Result<Terminal<CrosstermBackend<W>>> {
            let backend = backend::CrosstermBackend::new(self.writer);
            let backend = match self.raw_mode {
                RawMode::Enabled => backend.with_raw_mode()?,
                RawMode::Disabled => backend,
            };
            let backend = match self.alternate_screen {
                AlternateScreenMode::EnterAlternateScreen => backend.with_alternate_screen()?,
                AlternateScreenMode::MainScreen => backend,
            };
            let backend = match self.mouse_capture {
                MouseCapture::Captured => backend.with_mouse_capture()?,
                MouseCapture::NotCaptured => backend,
            };
            Terminal::new(backend)
        }
    }
}

#[cfg(feature = "termion")]
mod termion {
    use ::termion::{
        raw::{IntoRawMode, RawTerminal},
        screen::{AlternateScreen, IntoAlternateScreen, ToMainScreen},
    };
    use backend::TermionBackend;

    use super::*;

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct TerminalBuilder<W: Write> {
        writer: W,
        raw_mode: RawMode,
        alternate_screen: AlternateScreenMode,
        mouse_capture: MouseCapture,
    }

    impl<W: Write> TerminalBuilder<W> {
        pub fn new(writer: W) -> Self {
            TerminalBuilder::<W> {
                writer,
                raw_mode: RawMode::default(),
                alternate_screen: AlternateScreenMode::default(),
                mouse_capture: MouseCapture::default(),
            }
        }

        pub fn raw_mode(self, raw_mode: RawMode) -> TerminalBuilder<W> {
            TerminalBuilder::<W> { raw_mode, ..self }
        }

        pub fn alternate_screen(self, alternate_screen: AlternateScreenMode) -> Self {
            TerminalBuilder::<W> {
                alternate_screen,
                ..self
            }
        }

        pub fn mouse_capture(self, mouse_capture: MouseCapture) -> Self {
            TerminalBuilder::<W> {
                mouse_capture,
                ..self
            }
        }

        const DISABLE_MOUSE_CAPTURE: &'static str = "\x1B[?1006l\x1b[?1015l\x1b[?1002l\x1b[?1000l";

        pub fn build(
            self,
        ) -> io::Result<Terminal<TermionBackend<AlternateScreen<RawTerminal<W>>>>> {
            let writer = self.writer.into_raw_mode()?;
            if self.raw_mode == RawMode::Disabled {
                writer.suspend_raw_mode()?;
            }
            let mut writer = writer.into_alternate_screen()?;
            if self.alternate_screen == AlternateScreenMode::MainScreen {
                write!(writer, "{}", ToMainScreen)?;
            }
            if self.mouse_capture == MouseCapture::NotCaptured {
                write!(writer, "{}", Self::DISABLE_MOUSE_CAPTURE)?;
            }
            let backend = TermionBackend::new(writer);
            Terminal::new(backend)
        }
    }
}

#[cfg(feature = "termwiz")]
mod termwiz {
    use std::error::Error;

    use backend::TermwizBackend;

    use super::*;

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct TerminalBuilder {
        raw_mode: RawMode,
        alternate_screen: AlternateScreenMode,
    }

    impl TerminalBuilder {
        pub fn new() -> Self {
            TerminalBuilder::default()
        }

        pub fn raw_mode(self, raw_mode: RawMode) -> Self {
            TerminalBuilder { raw_mode, ..self }
        }

        pub fn alternate_screen(self, alternate_screen: AlternateScreenMode) -> Self {
            TerminalBuilder {
                alternate_screen,
                ..self
            }
        }

        pub fn build(self) -> Result<Terminal<TermwizBackend>, Box<dyn Error>> {
            let mut backend = TermwizBackend::new()?;
            match self.raw_mode {
                RawMode::Enabled => backend.enable_raw_mode()?,
                RawMode::Disabled => backend.disable_raw_mode()?,
            };
            match self.alternate_screen {
                AlternateScreenMode::EnterAlternateScreen => backend.enter_alternate_screen()?,
                AlternateScreenMode::MainScreen => backend.leave_alternate_screen()?,
            };
            let terminal = Terminal::new(backend)?;
            Ok(terminal)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    // the tests cannot run in parallel as the terminal is a global resource
    use serial_test::serial;

    use super::*;

    #[test]
    #[serial]
    #[cfg(feature = "crossterm")]
    fn crossterm() {
        struct Expected<W: Write> {
            writer: W,
            raw_mode_enabled: bool,
            alternate_screen_entered: bool,
            mouse_capture_enabled: bool,
        }
        #[track_caller]
        fn test<W: Write + Debug>(
            terminal_builder: crossterm::TerminalBuilder<W>,
            expected: Expected<W>,
        ) {
            let terminal = terminal_builder.build().unwrap();
            let backend = terminal.backend();
            assert_eq!(
                format!("{backend:?}"),
                format!(
                    "CrosstermBackend {{ writer: {:?}, raw_mode_enabled: {}, \
                    alternate_screen_entered: {}, mouse_capture_enabled: {} }}",
                    expected.writer,
                    expected.raw_mode_enabled,
                    expected.alternate_screen_entered,
                    expected.mouse_capture_enabled
                )
            );
        }

        test(
            TerminalBuilder::crossterm(stdout()),
            Expected {
                writer: stdout(),
                raw_mode_enabled: true,
                alternate_screen_entered: true,
                mouse_capture_enabled: false,
            },
        );
        test(
            TerminalBuilder::crossterm(stderr()),
            Expected {
                writer: stderr(),
                raw_mode_enabled: true,
                alternate_screen_entered: true,
                mouse_capture_enabled: false,
            },
        );
        test(
            TerminalBuilder::crossterm_on_stdout(),
            Expected {
                writer: stdout(),
                raw_mode_enabled: true,
                alternate_screen_entered: true,
                mouse_capture_enabled: false,
            },
        );
        test(
            TerminalBuilder::crossterm_on_stderr(),
            Expected {
                writer: stderr(),
                raw_mode_enabled: true,
                alternate_screen_entered: true,
                mouse_capture_enabled: false,
            },
        );
        test(
            TerminalBuilder::crossterm_on_stdout()
                .raw_mode(RawMode::Enabled)
                .alternate_screen(AlternateScreenMode::EnterAlternateScreen)
                .mouse_capture(MouseCapture::Captured),
            Expected {
                writer: stdout(),
                raw_mode_enabled: true,
                alternate_screen_entered: true,
                mouse_capture_enabled: true,
            },
        );
        test(
            TerminalBuilder::crossterm_on_stderr()
                .raw_mode(RawMode::Disabled)
                .alternate_screen(AlternateScreenMode::MainScreen)
                .mouse_capture(MouseCapture::NotCaptured),
            Expected {
                writer: stderr(),
                raw_mode_enabled: false,
                alternate_screen_entered: false,
                mouse_capture_enabled: false,
            },
        );
    }

    #[test]
    #[serial]
    #[cfg(feature = "termion")]
    fn termion() -> io::Result<()> {
        // due to the way termion works, we cannot test the actual terminal, but we can check that
        // this compiles and runs without errors
        let _ = TerminalBuilder::termion(stdout()).build()?;
        let _ = TerminalBuilder::termion(stderr()).build()?;
        let _ = TerminalBuilder::termion_on_stdout().build()?;
        let _ = TerminalBuilder::termion_on_stderr().build()?;
        let _ = TerminalBuilder::termion_on_stdout()
            .raw_mode(RawMode::Enabled)
            .alternate_screen(AlternateScreenMode::EnterAlternateScreen)
            .mouse_capture(MouseCapture::Captured)
            .build()?;
        let _ = TerminalBuilder::termion_on_stderr()
            .raw_mode(RawMode::Disabled)
            .alternate_screen(AlternateScreenMode::MainScreen)
            .mouse_capture(MouseCapture::NotCaptured)
            .build()?;

        Ok(())
    }

    #[test]
    #[serial]
    #[cfg(feature = "termwiz")]
    fn termwiz() -> Result<(), Box<dyn std::error::Error>> {
        // due to the way termwiz works, we cannot test the actual terminal, but we can check that
        // this compiles and runs without errors
        let _ = TerminalBuilder::termwiz_on_stdout().build()?;
        let _ = TerminalBuilder::termwiz_on_stdout()
            .raw_mode(RawMode::Enabled)
            .alternate_screen(AlternateScreenMode::EnterAlternateScreen)
            .build()?;
        let _ = TerminalBuilder::termwiz_on_stdout()
            .raw_mode(RawMode::Disabled)
            .alternate_screen(AlternateScreenMode::MainScreen)
            .build()?;
        Ok(())
    }
}
