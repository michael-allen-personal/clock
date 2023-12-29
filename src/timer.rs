use std::io::BufReader;
use std::time::{Duration, Instant};

use rodio::{OutputStream, Sink};

use crate::app;
use crate::clock;

enum TimerState {
    SetTimer(clock::TimeDisplay),
    Countdown(Instant, clock::TimeDisplay),
    PlayingTimerSound(clock::TimeDisplay, OutputStream, Sink),
}

impl TimerState {
    fn set_timer(time_display: clock::TimeDisplay) -> Self {
        TimerState::SetTimer(time_display)
    }

    fn start_countdown(time_display: clock::TimeDisplay) -> Self {
        TimerState::Countdown(Instant::now(), time_display)
    }

    fn play_timer_sound(time_display: clock::TimeDisplay) -> Self {
        // get the default device every time, as sometimes I will change
        // output while the clock is still active. I would want the timer
        // to play out of whatever the current output device is when it
        // goes off
        let (stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        // something better needs to be done with this
        let timer_sound_file =
            std::fs::File::open("/home/michael/.clock/FinalFantasyVictoryFanfareOrchestrated.flac")
                .unwrap();

        sink.append(rodio::Decoder::new(BufReader::new(timer_sound_file)).unwrap());

        TimerState::PlayingTimerSound(time_display, stream, sink)
    }

    fn stop_timer(&mut self) {
        if let TimerState::PlayingTimerSound(time_display, _stream, sink) = self {
            sink.stop();
            *self = TimerState::SetTimer(*time_display);
        }
    }
}

pub struct Timer {
    timer_state: TimerState,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            timer_state: TimerState::SetTimer(clock::TimeDisplay::default()),
        }
    }
}

// i32 can hold almost 600 hours in ms, no realistic need to worry about overflow
#[allow(clippy::cast_possible_truncation)]
impl app::Ui for Timer {
    fn ui(&mut self, ui: &mut egui::Ui) {
        match &mut self.timer_state {
            TimerState::SetTimer(time_display) => {
                time_display.ui_hms_input(ui);
                if ui.button("Start").clicked() {
                    self.timer_state = TimerState::start_countdown(*time_display);
                }
                if ui.button("Reset").clicked() {
                    self.timer_state = TimerState::set_timer(clock::TimeDisplay::default());
                }
            }
            TimerState::Countdown(start_time, time_display) => {
                // .5 seconds in nanoseconds
                // Make sure the window is refershing less than once a second so the second
                // countdown looks smooth
                ui.ctx()
                    .request_repaint_after(Duration::new(0, 500_000_000));
                // Use miliseconds so the visual update is less likely to be choppy
                // due to rounding. Honestly didn't fully check to see if it works
                // that way but it makes sense that it would as these arent float types
                let elapsed = start_time.elapsed().as_millis() as i32;
                let remaining_ms = time_display.as_sec_i32() * 1000 - elapsed;
                if remaining_ms <= 0 {
                    self.timer_state = TimerState::play_timer_sound(*time_display);
                    return;
                }

                ui.label(time_left_as_str(remaining_ms / 1000));
                if ui.button("Stop").clicked() {
                    self.timer_state = TimerState::start_countdown(*time_display);
                }
            }
            TimerState::PlayingTimerSound(_clock_value, _stream, _sink) => {
                ui.label("Times Up!");
                if ui.button("Stop").clicked() {
                    self.timer_state.stop_timer();
                }
            }
        }
    }
}

fn time_left_as_str(remaining_sec: i32) -> String {
    // Convert remaining_sec to hours, minutes, and seconds
    let hours = remaining_sec / 3600;
    let minutes = (remaining_sec % 3600) / 60;
    let seconds = remaining_sec % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}
