use super::*;

#[derive(Debug)]
pub struct AssetsManager 
{
}

impl Default for AssetsManager
{
    fn default() -> Self { Self::const_default() }
}

impl AssetsManager
{
    pub const FONT_DEFAULT_SIZE : FontSize = 96;

    pub(crate) const fn const_default() -> Self { Self {  }}
}

#[derive(Debug)]
pub enum AssetImportError<T : ImportFromRaw>
{
    File(AssetFileError),
    Raw(T::ImportRawError),
}

pub type AssetFileError = macroquad::Error;

pub trait ImportFromFile : Sized + ImportFromRaw
{
    #[allow(async_fn_in_trait)]
    async fn from_file(path : &str) -> Result<Self, AssetImportError<Self>> 
    {
        match macroquad::file::load_file(path).await
        {
            Ok(raw) => Self::from_raw(&raw).map_err(|e| AssetImportError::Raw(e)),
            Err(e) => Err(AssetImportError::File(e)),
        }
    }
}
impl<T> ImportFromFile for T where T : Sized + ImportFromRaw {} 

pub trait ImportFromRaw : Sized
{
    type ImportRawError;
    fn from_raw(raw : &[u8]) -> Result<Self, Self::ImportRawError>;
}