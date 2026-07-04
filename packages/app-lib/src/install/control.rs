//! Job control: cancellation and pause support for install jobs.

use dashmap::DashMap;
use std::sync::LazyLock;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

/// Per-job control handles: cancellation token + pause signal.
#[derive(Clone)]
pub struct JobControl {
    pub cancel: CancellationToken,
    pub pause_rx: watch::Receiver<bool>,
    pause_tx: watch::Sender<bool>,
}

impl JobControl {
    pub fn new() -> Self {
        let (pause_tx, pause_rx) = watch::channel(false);
        Self {
            cancel: CancellationToken::new(),
            pause_rx,
            pause_tx,
        }
    }

    /// Request the job to pause.
    pub fn pause(&self) {
        let _ = self.pause_tx.send(true);
    }

    /// Request the job to resume.
    pub fn resume(&self) {
        let _ = self.pause_tx.send(false);
    }

    /// Check if the job is currently paused (non-blocking).
    pub fn is_paused(&self) -> bool {
        *self.pause_rx.borrow()
    }
}

/// Global registry mapping job IDs to their control handles.
static JOB_CONTROLS: LazyLock<DashMap<Uuid, JobControl>> =
    LazyLock::new(DashMap::new);

/// Create and register control handles for a new job.
pub fn register_job(job_id: Uuid) -> JobControl {
    let control = JobControl::new();
    JOB_CONTROLS.insert(job_id, control.clone());
    control
}

/// Get control handles for a job (if it exists).
pub fn get_control(job_id: &Uuid) -> Option<JobControl> {
    JOB_CONTROLS.get(job_id).map(|r| r.value().clone())
}

/// Remove control handles when a job finishes.
pub fn unregister_job(job_id: &Uuid) {
    JOB_CONTROLS.remove(job_id);
}

/// Trigger cancellation for a running job. Returns true if the job was found.
pub fn request_cancel(job_id: &Uuid) -> bool {
    if let Some(control) = JOB_CONTROLS.get(job_id) {
        control.cancel.cancel();
        true
    } else {
        false
    }
}

/// Trigger pause for a running job. Returns true if the job was found.
pub fn request_pause(job_id: &Uuid) -> bool {
    if let Some(control) = JOB_CONTROLS.get(job_id) {
        control.pause();
        true
    } else {
        false
    }
}

/// Trigger resume for a paused job. Returns true if the job was found.
pub fn request_resume(job_id: &Uuid) -> bool {
    if let Some(control) = JOB_CONTROLS.get(job_id) {
        control.resume();
        true
    } else {
        false
    }
}

/// A wrapper that checks for cancellation and pause on behalf of a job.
/// Insert `.await?` calls in download loops to make them interruptible.
#[derive(Clone)]
pub struct JobGuard {
    cancel: CancellationToken,
    pause_rx: watch::Receiver<bool>,
}

impl JobGuard {
    pub fn new(control: &JobControl) -> Self {
        Self {
            cancel: control.cancel.clone(),
            pause_rx: control.pause_rx.clone(),
        }
    }

    /// Check for cancellation and pause. Returns an error if cancelled.
    /// While paused, this will sleep until resumed.
    pub async fn check(&mut self) -> crate::Result<()> {
        // If cancelled, return error
        if self.cancel.is_cancelled() {
            return Err(crate::ErrorKind::InputError(
                "Install was canceled".to_string(),
            )
            .into());
        }

        // If paused, wait until resumed or cancelled
        while *self.pause_rx.borrow() {
            // Check cancellation while paused
            if self.cancel.is_cancelled() {
                return Err(crate::ErrorKind::InputError(
                    "Install was canceled".to_string(),
                )
                .into());
            }
            // Wait for pause state change or cancellation
            tokio::select! {
                _ = self.cancel.cancelled() => {
                    return Err(crate::ErrorKind::InputError(
                        "Install was canceled".to_string(),
                    ).into());
                }
                result = self.pause_rx.changed() => {
                    if result.is_err() {
                        // Sender dropped, treat as resume
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    /// Quick non-async check if cancelled (for tight loops).
    pub fn is_cancelled(&self) -> bool {
        self.cancel.is_cancelled()
    }
}
