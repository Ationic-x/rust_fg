//! Este módulo principal configura y ejecuta la ventana principal del juego.
//! 
//! El juego utiliza la biblioteca `piston_window` para manejar la ventana y
//! los eventos del juego. La lógica del juego se organiza a través de
//! diferentes módulos y el `ScreenManager` se encarga de gestionar las
//! diferentes pantallas del juego.
mod player;
mod chars;
pub mod views;
mod preloader;
mod error;

/// Enumaración de teclas de comandos que se pueden pulsar
#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum CK {
    DB,
    D,
    DF,
    B,
    F,
    UB,
    U,
    UF,
    LP,
    MP,
    HP,
    LK,
    MK,
    HK,
    Start,
}