pub(crate) use bevy::prelude::*;
pub(crate) use bevy::sprite::*;
pub(crate) use bevy_editor_pls::prelude::*;
pub(crate) use bevy_turborand::prelude::*;
pub(crate) use leafwing_input_manager::prelude::*;

pub(crate) use crate::{body::Body, cleanup, motion::Motion};

pub(crate) mod body {
    #[allow(unused_imports)]
    pub(crate) use crate::body::prelude::*;
}

pub(crate) mod camera {
    #[allow(unused_imports)]
    pub(crate) use crate::camera::prelude::*;
}

pub(crate) mod motion {
    #[allow(unused_imports)]
    pub(crate) use crate::motion::prelude::*;
}

pub(crate) mod player {
    #[allow(unused_imports)]
    pub(crate) use crate::player::prelude::*;
}
