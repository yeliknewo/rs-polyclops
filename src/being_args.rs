use math::{Vec3};

#[derive(Clone)]
pub struct BeingArgs {
    pub pos: Option<Box<Vec3>>,
    pub vel: Option<Box<Vec3>>,
    pub acc: Option<Box<Vec3>>,
    pub sca: Option<Box<Vec3>>,
    pub rot: Option<Box<Vec3>>,
}

impl BeingArgs {
    pub fn new() -> BeingArgs {
        BeingArgs {
            pos: None,
            vel: None,
            acc: None,
            sca: None,
            rot: None,
        }
    }

    pub fn with_pos(mut self, vec3: Vec3) -> BeingArgs {
        self.pos = Some(Box::new(vec3));
        self
    }

    pub fn with_vel(mut self, vec3: Vec3) -> BeingArgs {
        self.vel = Some(Box::new(vec3));
        self
    }

    pub fn with_acc(mut self, vec3: Vec3) -> BeingArgs {
        self.acc = Some(Box::new(vec3));
        self
    }

    pub fn with_sca(mut self, vec3: Vec3) -> BeingArgs {
        self.sca = Some(Box::new(vec3));
        self
    }

    pub fn with_rot(mut self, vec3: Vec3) -> BeingArgs {
        self.rot = Some(Box::new(vec3));
        self
    }
}
