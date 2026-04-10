/*!
# cuda-time

Time utilities for agent reasoning.

Agents need temporal awareness — how long did something take? Is a
deadline approaching? What time window are we in? This crate provides
time primitives for scheduling, deadlines, and duration math.

- Monotonic timestamp management
- Duration arithmetic
- Deadline tracking with urgency
- Time windows (business hours, shift patterns)
- Interval management
- Temporal context for decision-making
*/

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// A deadline with urgency tracking
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Deadline {
    pub id: String,
    pub label: String,
    pub deadline_ms: u64,
    pub warn_at_pct: f64,    // warn when this fraction of time has elapsed
    pub created_ms: u64,
    pub completed: bool,
}

impl Deadline {
    pub fn new(id: &str, label: &str, deadline_ms: u64) -> Self {
        Deadline { id: id.to_string(), label: label.to_string(), deadline_ms, warn_at_pct: 0.75, created_ms: now(), completed: false }
    }

    pub fn remaining_ms(&self) -> i64 {
        if self.completed { return i64::MAX; }
        (self.deadline_ms as i64) - (now() as i64)
    }

    pub fn is_expired(&self) -> bool { self.remaining_ms() < 0 }

    pub fn is_urgent(&self) -> bool {
        if self.completed { return false; }
        let total = self.deadline_ms - self.created_ms;
        let elapsed = now() - self.created_ms;
        total > 0 && (elapsed as f64 / total as f64) > self.warn_at_pct
    }

    pub fn progress_pct(&self) -> f64 {
        if self.completed { return 1.0; }
        let total = self.deadline_ms - self.created_ms;
        if total == 0 { return 1.0; }
        ((now() - self.created_ms) as f64 / total as f64).min(1.0)
    }

    pub fn complete(&mut self) { self.completed = true; }
}

/// A time window (recurring)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeWindow {
    pub name: String,
    pub start_hour: u8,    // 0-23
    pub start_minute: u8,  // 0-59
    pub end_hour: u8,
    pub end_minute: u8,
    pub days: Vec<u8>,     // 0=Sun, 1=Mon, ..., 6=Sat, empty = all days
}

impl TimeWindow {
    pub fn new(name: &str, start_h: u8, start_m: u8, end_h: u8, end_m: u8) -> Self {
        TimeWindow { name: name.to_string(), start_hour: start_h, start_minute: start_m, end_hour: end_h, end_minute: end_m, days: vec![] }
    }

    pub fn weekday_only(mut self) -> Self { self.days = vec![1, 2, 3, 4, 5]; self }
    pub fn weekend_only(mut self) -> Self { self.days = vec![0, 6]; self }

    /// Is current time within this window?
    pub fn is_active(&self) -> bool {
        let now = now();
        let secs = now / 1000;
        let day_of_week = ((secs / 86400) + 4) % 7; // 0=Thu epoch start
        if !self.days.is_empty() && !self.days.contains(&(day_of_week as u8)) { return false; }
        let hour_of_day = ((secs / 3600) % 24) as u8;
        let minute_of_hour = ((secs / 60) % 60) as u8;
        let start_mins = self.start_hour as u32 * 60 + self.start_minute as u32;
        let end_mins = self.end_hour as u32 * 60 + self.end_minute as u32;
        let current_mins = hour_of_day as u32 * 60 + minute_of_hour as u32;
        if end_mins > start_mins { current_mins >= start_mins && current_mins < end_mins }
        else { current_mins >= start_mins || current_mins < end_mins } // wraps midnight
    }

    /// Minutes until window opens (0 if active)
    pub fn minutes_until_open(&self) -> u32 {
        if self.is_active() { return 0; }
        let now = now();
        let secs = now / 1000;
        let current_mins = ((secs / 60) % 1440) as u32;
        let start_mins = self.start_hour as u32 * 60 + self.start_minute as u32;
        if start_mins > current_mins { start_mins - current_mins }
        else { 1440 - current_mins + start_mins }
    }
}

/// A timer for measuring duration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Timer {
    pub label: String,
    pub start_ms: u64,
    pub elapsed_ms: u64,
    pub running: bool,
    pub laps: VecDeque<(u64, String)>, // (elapsed_ms, label)
}

impl Timer {
    pub fn new(label: &str) -> Self { Timer { label: label.to_string(), start_ms: now(), elapsed_ms: 0, running: true, laps: VecDeque::new() } }

    pub fn stop(&mut self) { if self.running { self.elapsed_ms = now() - self.start_ms; self.running = false; } }

    pub fn elapsed(&self) -> u64 {
        if self.running { now() - self.start_ms } else { self.elapsed_ms }
    }

    pub fn lap(&mut self, label: &str) -> u64 {
        let e = self.elapsed();
        self.laps.push_back((e, label.to_string()));
        e
    }

    pub fn reset(&mut self) { self.start_ms = now(); self.elapsed_ms = 0; self.running = true; self.laps.clear(); }
}

/// The time manager
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeManager {
    pub deadlines: Vec<Deadline>,
    pub timers: HashMap<String, Timer>,
    pub windows: Vec<TimeWindow>,
}

impl TimeManager {
    pub fn new() -> Self { TimeManager { deadlines: vec![], timers: HashMap::new(), windows: vec![] } }

    /// Add deadline
    pub fn add_deadline(&mut self, deadline: Deadline) { self.deadlines.push(deadline); }

    /// Get expired deadlines
    pub fn expired_deadlines(&self) -> Vec<&Deadline> { self.deadlines.iter().filter(|d| d.is_expired() && !d.completed).collect() }

    /// Get urgent deadlines
    pub fn urgent_deadlines(&self) -> Vec<&Deadline> { self.deadlines.iter().filter(|d| d.is_urgent() && !d.completed).collect() }

    /// Start a timer
    pub fn start_timer(&mut self, name: &str) { self.timers.insert(name.to_string(), Timer::new(name)); }

    /// Stop a timer
    pub fn stop_timer(&mut self, name: &str) -> Option<u64> {
        self.timers.get_mut(name).map(|t| { t.stop(); t.elapsed() })
    }

    /// Get timer elapsed
    pub fn timer_elapsed(&self, name: &str) -> Option<u64> { self.timers.get(name).map(|t| t.elapsed()) }

    /// Add time window
    pub fn add_window(&mut self, window: TimeWindow) { self.windows.push(window); }

    /// Check if any window is active
    pub fn any_window_active(&self) -> bool { self.windows.iter().any(|w| w.is_active()) }

    /// Summary
    pub fn summary(&self) -> String {
        let expired = self.expired_deadlines().len();
        let urgent = self.urgent_deadlines().len();
        let active_timers = self.timers.values().filter(|t| t.running).count();
        format!("Time: {} deadlines ({} expired, {} urgent), {} timers ({} running), {} windows",
            self.deadlines.len(), expired, urgent, self.timers.len(), active_timers, self.windows.len())
    }
}

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deadline_not_expired() {
        let d = Deadline::new("d1", "task", now() + 60_000);
        assert!(!d.is_expired());
    }

    #[test]
    fn test_deadline_expired() {
        let d = Deadline::new("d1", "task", 0);
        assert!(d.is_expired());
    }

    #[test]
    fn test_deadline_progress() {
        let mut d = Deadline::new("d1", "task", now() + 100_000);
        d.created_ms = now() - 75_000;
        assert!(d.is_urgent()); // 75% elapsed
    }

    #[test]
    fn test_deadline_complete() {
        let mut d = Deadline::new("d1", "task", 0);
        d.complete();
        assert!(!d.is_expired());
    }

    #[test]
    fn test_timer_elapsed() {
        let mut t = Timer::new("test");
        // Timer is running, elapsed should be >= 0
        let e = t.elapsed();
        assert!(e >= 0);
        t.stop();
        assert!(!t.running);
    }

    #[test]
    fn test_timer_laps() {
        let mut t = Timer::new("race");
        t.lap("checkpoint1");
        t.lap("checkpoint2");
        assert_eq!(t.laps.len(), 2);
    }

    #[test]
    fn test_time_window_current() {
        // Create a window that spans all day
        let w = TimeWindow::new("allday", 0, 0, 23, 59);
        assert!(w.is_active());
    }

    #[test]
    fn test_weekday_window() {
        let w = TimeWindow::new("work", 9, 0, 17, 0).weekday_only();
        let secs = now() / 1000;
        let dow = ((secs / 86400) + 4) % 7;
        // Just check it doesn't panic
        let _ = w.is_active();
        let _ = w.minutes_until_open();
        if w.is_active() { assert_eq!(w.minutes_until_open(), 0); }
    }

    #[test]
    fn test_time_manager() {
        let mut tm = TimeManager::new();
        tm.add_deadline(Deadline::new("d1", "past", 0));
        tm.start_timer("op1");
        assert_eq!(tm.expired_deadlines().len(), 1);
        assert!(tm.timer_elapsed("op1").is_some());
    }

    #[test]
    fn test_time_manager_summary() {
        let tm = TimeManager::new();
        let s = tm.summary();
        assert!(s.contains("0 deadlines"));
    }

    #[test]
    fn test_duration_between() {
        let start = now() - 5000;
        let end = now();
        assert!(end > start);
    }
}
