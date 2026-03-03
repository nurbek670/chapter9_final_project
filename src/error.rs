// error.rs - ilova uchun yagona xato tipi
// barcha xato turlarni birlashtiradi (? operatori uchun from() lozim)
use std::{fmt, io};

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),           // file operatsiyasi xatosi
    InvalidUsername(String), // noto'g'ri foydalanuvchi nomi
    UserNotFound(String),    // foydalanuvchi topilmadi
    EmptyDatabase,           // malumotlar bazasi bo'sh
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO xatosi: {}", e),
            AppError::InvalidUsername(u) => write!(f, "Noto'g'ri ism: '{}'", u),
            AppError::UserNotFound(u) => write!(f, "topilmadi: '{}'", u),
            AppError::EmptyDatabase => write!(f, "malumotlar bazasi bo'sh"),
        }
    }
}

// io::Error -> AppError konvertatsiya (? operatori uchun zarur)
impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}
