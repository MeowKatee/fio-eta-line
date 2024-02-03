use rust_decimal::Decimal;
use std::{
    ops::{Index, IndexMut},
    time::Duration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct JobStatuses {
    pub initialized: u32,
    pub pre_reading: u32,
    pub ramp_period: u32,
    pub seq_read: u32,
    pub rand_read: u32,
    pub seq_write: u32,
    pub rand_write: u32,
    pub seq_trim: u32,
    pub rand_trim: u32,
    pub mixed_seq_reads_writes: u32,
    pub mixed_rand_reads_writes: u32,
    pub waiting_fsync: u32,
    pub verify_written_data: u32,
    pub finishing: u32,
    pub exited_not_reaped: u32,
    pub reaped: u32,
    pub exited_with_error: u32,
    pub exited_due_to_signal: u32,
    pub unknown: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum JobStatus {
    Initialized,
    PreReading,
    RampPeriod,
    SeqRead,
    RandRead,
    SeqWrite,
    RandWrite,
    SeqTrim,
    RandTrim,
    MixedSeqReadsWrites,
    MixedRandReadsWrites,
    WaitingFsync,
    VerifyWrittenData,
    Finishing,
    ExitedNotReaped,
    Reaped,
    ExitedWithError,
    ExitedDueToSignal,
    Unknown(char),
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct FioEtaLine {
    pub jobs_unfinished: u32,
    pub opened_files: u32,
    pub rate_limit: Option<String>,
    pub job_statuses: JobStatuses,
    pub progress_percentage: Decimal,
    pub read_speed: Option<String>,
    pub write_speed: Option<String>,
    pub trim_speed: Option<String>,
    pub read_iops: Option<String>,
    pub write_iops: Option<String>,
    pub trim_iops: Option<String>,
    pub eta: Duration,
}

impl Into<JobStatus> for char {
    fn into(self) -> JobStatus {
        match self {
            'I' => JobStatus::Initialized,
            'p' => JobStatus::PreReading,
            '/' => JobStatus::RampPeriod,
            'R' => JobStatus::SeqRead,
            'r' => JobStatus::RandRead,
            'W' => JobStatus::SeqWrite,
            'w' => JobStatus::RandWrite,
            'D' => JobStatus::SeqTrim,
            'd' => JobStatus::RandTrim,
            'M' => JobStatus::MixedSeqReadsWrites,
            'm' => JobStatus::MixedRandReadsWrites,
            'F' => JobStatus::WaitingFsync,
            'V' => JobStatus::VerifyWrittenData,
            'f' => JobStatus::Finishing,
            'E' => JobStatus::ExitedNotReaped,
            '_' => JobStatus::Reaped,
            'X' => JobStatus::ExitedWithError,
            'K' => JobStatus::ExitedDueToSignal,
            _ => JobStatus::Unknown(self),
        }
    }
}

impl Index<JobStatus> for JobStatuses {
    type Output = u32;

    fn index(&self, index: JobStatus) -> &Self::Output {
        match index {
            JobStatus::Initialized => &self.initialized,
            JobStatus::PreReading => &self.pre_reading,
            JobStatus::RampPeriod => &self.ramp_period,
            JobStatus::SeqRead => &self.seq_read,
            JobStatus::RandRead => &self.rand_read,
            JobStatus::SeqWrite => &self.seq_write,
            JobStatus::RandWrite => &self.rand_write,
            JobStatus::SeqTrim => &self.seq_trim,
            JobStatus::RandTrim => &self.rand_trim,
            JobStatus::MixedSeqReadsWrites => &self.mixed_seq_reads_writes,
            JobStatus::MixedRandReadsWrites => &self.mixed_rand_reads_writes,
            JobStatus::WaitingFsync => &self.waiting_fsync,
            JobStatus::VerifyWrittenData => &self.verify_written_data,
            JobStatus::Finishing => &self.finishing,
            JobStatus::ExitedNotReaped => &self.exited_not_reaped,
            JobStatus::Reaped => &self.reaped,
            JobStatus::ExitedWithError => &self.exited_with_error,
            JobStatus::ExitedDueToSignal => &self.exited_due_to_signal,
            JobStatus::Unknown(_) => &self.unknown,
        }
    }
}

impl IndexMut<JobStatus> for JobStatuses {
    fn index_mut(&mut self, index: JobStatus) -> &mut Self::Output {
        match index {
            JobStatus::Initialized => &mut self.initialized,
            JobStatus::PreReading => &mut self.pre_reading,
            JobStatus::RampPeriod => &mut self.ramp_period,
            JobStatus::SeqRead => &mut self.seq_read,
            JobStatus::RandRead => &mut self.rand_read,
            JobStatus::SeqWrite => &mut self.seq_write,
            JobStatus::RandWrite => &mut self.rand_write,
            JobStatus::SeqTrim => &mut self.seq_trim,
            JobStatus::RandTrim => &mut self.rand_trim,
            JobStatus::MixedSeqReadsWrites => &mut self.mixed_seq_reads_writes,
            JobStatus::MixedRandReadsWrites => &mut self.mixed_rand_reads_writes,
            JobStatus::WaitingFsync => &mut self.waiting_fsync,
            JobStatus::VerifyWrittenData => &mut self.verify_written_data,
            JobStatus::Finishing => &mut self.finishing,
            JobStatus::ExitedNotReaped => &mut self.exited_not_reaped,
            JobStatus::Reaped => &mut self.reaped,
            JobStatus::ExitedWithError => &mut self.exited_with_error,
            JobStatus::ExitedDueToSignal => &mut self.exited_due_to_signal,
            JobStatus::Unknown(_) => &mut self.unknown,
        }
    }
}

pub fn fold_job_statuses(
    job_status_iter: impl IntoIterator<Item = (JobStatus, u32)>,
) -> JobStatuses {
    let mut job_statuses = JobStatuses::default();
    for (job_status, count) in job_status_iter {
        job_statuses[job_status] += count;
    }
    job_statuses
}
