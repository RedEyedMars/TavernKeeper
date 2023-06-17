use super::Glyph;
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Status {
    Barrier(Glyph),
    Burning,
    Stunned,
    Submerged,
    Shocked,
    Weakened,
    Raging,
    Hardened,
    Fluid,
    Flying,
}

#[derive(Clone, Debug)]
pub struct StatusSet {
    pub(in super::super) barrier_fire: (u16, u16),
    pub(in super::super) barrier_air: (u16, u16),
    pub(in super::super) barrier_earth: (u16, u16),
    pub(in super::super) barrier_water: (u16, u16),
    pub(in super::super) barrier_void: (u16, u16),
    pub(in super::super) burning: (u16, u16),
    pub(in super::super) stunned: (u16, u16),
    pub(in super::super) submerged: (u16, u16),
    pub(in super::super) shocked: (u16, u16),
    pub(in super::super) weakened: (u16, u16),
    pub(in super::super) raging: (u16, u16),
    pub(in super::super) hardened: (u16, u16),
    pub(in super::super) fluid: (u16, u16),
    pub(in super::super) flying: (u16, u16),
}

impl StatusSet {
    pub fn new() -> Self {
        StatusSet {
            barrier_fire: (0, 0),
            barrier_air: (0, 0),
            barrier_earth: (0, 0),
            barrier_water: (0, 0),
            barrier_void: (0, 0),
            burning: (0, 0),
            stunned: (0, 0),
            submerged: (0, 0),
            shocked: (0, 0),
            weakened: (0, 0),
            raging: (0, 0),
            hardened: (0, 0),
            fluid: (0, 0),
            flying: (0, 0),
        }
    }

    pub fn insert(&mut self, status: &Status, value: u16, duration: u16) {
        match status {
            Status::Barrier(Glyph::Fire) => {
                if value * duration > self.barrier_fire.0 * self.barrier_fire.1 {
                    self.barrier_fire = (value, duration)
                }
            },
            Status::Barrier(Glyph::Air) => {
                if value * duration > self.barrier_air.0 * self.barrier_air.1 {
                    self.barrier_air = (value, duration)
                }
            },
            Status::Barrier(Glyph::Earth) => {
                if value * duration > self.barrier_earth.0 * self.barrier_earth.1 {
                    self.barrier_earth = (value, duration)
                }
            },
            Status::Barrier(Glyph::Water) => {
                if value * duration > self.barrier_water.0 * self.barrier_water.1 {
                    self.barrier_water = (value, duration)
                }
            },
            Status::Barrier(Glyph::Void) => {
                if value * duration > self.barrier_void.0 * self.barrier_void.1 {
                    self.barrier_void = (value, duration)
                }
            },
            Status::Burning => {
                if value * duration > self.burning.0 * self.burning.1 {
                    self.burning = (value, duration)
                }
            },
            Status::Stunned => {
                if value * duration > self.stunned.0 * self.stunned.1 {
                    self.stunned = (value, duration)
                }
            },
            Status::Submerged => {
                if value * duration > self.submerged.0 * self.submerged.1 {
                    self.submerged = (value, duration)
                }
            },
            Status::Shocked => {
                if value * duration > self.shocked.0 * self.shocked.1 {
                    self.shocked = (value, duration)
                }
            },
            Status::Weakened => {
                if value * duration > self.weakened.0 * self.weakened.1 {
                    self.weakened = (value, duration)
                }
            },
            Status::Raging => {
                if value * duration > self.raging.0 * self.raging.1 {
                    self.raging = (value, duration)
                }
            },
            Status::Hardened => {
                if value * duration > self.hardened.0 * self.hardened.1 {
                    self.hardened = (value, duration)
                }
            },
            Status::Fluid => {
                if value * duration > self.fluid.0 * self.fluid.1 {
                    self.fluid = (value, duration)
                }
            },
            Status::Flying => {
                if value * duration > self.flying.0 * self.flying.1 {
                    self.flying = (value, duration)
                }
            },
        }
    }

    pub fn value(&self, status: &Status) -> u16 {
        match status {
            Status::Barrier(Glyph::Fire) => self.barrier_fire.0,
            Status::Barrier(Glyph::Air) => self.barrier_air.0,
            Status::Barrier(Glyph::Earth) => self.barrier_earth.0,
            Status::Barrier(Glyph::Water) => self.barrier_water.0,
            Status::Barrier(Glyph::Void) => self.barrier_void.0,
            Status::Burning => self.burning.0,
            Status::Stunned => self.stunned.0,
            Status::Submerged => self.submerged.0,
            Status::Shocked => self.shocked.0,
            Status::Weakened => self.weakened.0,
            Status::Raging => self.raging.0,
            Status::Hardened => self.hardened.0,
            Status::Fluid => self.fluid.0,
            Status::Flying => self.flying.0,
        }
    }

    pub fn entry(&self, status: &Status) -> Option<u16> {
        let val = self.value(status);
        if val == 0 {
            None
        } else {
            Some(val)
        }
    }

    pub fn duration(&self, status: &Status) -> u16 {
        match status {
            Status::Barrier(Glyph::Fire) => self.barrier_fire.1,
            Status::Barrier(Glyph::Air) => self.barrier_air.1,
            Status::Barrier(Glyph::Earth) => self.barrier_earth.1,
            Status::Barrier(Glyph::Water) => self.barrier_water.1,
            Status::Barrier(Glyph::Void) => self.barrier_void.1,
            Status::Burning => self.burning.1,
            Status::Stunned => self.stunned.1,
            Status::Submerged => self.submerged.1,
            Status::Shocked => self.shocked.1,
            Status::Weakened => self.weakened.1,
            Status::Raging => self.raging.1,
            Status::Hardened => self.hardened.1,
            Status::Fluid => self.fluid.1,
            Status::Flying => self.flying.1,
        }
    }

    pub fn remove(&mut self, status: &Status) {
        match status {
            Status::Barrier(Glyph::Fire) => self.barrier_fire = (0, 0),
            Status::Barrier(Glyph::Air) => self.barrier_air = (0, 0),
            Status::Barrier(Glyph::Earth) => self.barrier_earth = (0, 0),
            Status::Barrier(Glyph::Water) => self.barrier_water = (0, 0),
            Status::Barrier(Glyph::Void) => self.barrier_void = (0, 0),
            Status::Burning => self.burning = (0, 0),
            Status::Stunned => self.stunned = (0, 0),
            Status::Submerged => self.submerged = (0, 0),
            Status::Shocked => self.shocked = (0, 0),
            Status::Weakened => self.weakened = (0, 0),
            Status::Raging => self.raging = (0, 0),
            Status::Hardened => self.hardened = (0, 0),
            Status::Fluid => self.fluid = (0, 0),
            Status::Flying => self.flying = (0, 0),
        }
    }

    pub fn has(&self, status: &Status) -> bool {
        match status {
            Status::Barrier(Glyph::Fire) => self.barrier_fire.1 != 0,
            Status::Barrier(Glyph::Air) => self.barrier_air.1 != 0,
            Status::Barrier(Glyph::Earth) => self.barrier_earth.1 != 0,
            Status::Barrier(Glyph::Water) => self.barrier_water.1 != 0,
            Status::Barrier(Glyph::Void) => self.barrier_void.1 != 0,
            Status::Burning => self.burning.1 != 0,
            Status::Stunned => self.stunned.1 != 0,
            Status::Submerged => self.submerged.1 != 0,
            Status::Shocked => self.shocked.1 != 0,
            Status::Weakened => self.weakened.1 != 0,
            Status::Raging => self.raging.1 != 0,
            Status::Hardened => self.hardened.1 != 0,
            Status::Fluid => self.fluid.1 != 0,
            Status::Flying => self.flying.1 != 0,
        }
    }

    pub fn tick(&mut self, status: &Status) -> bool {
        match status {
            Status::Barrier(Glyph::Fire) => {
                if self.barrier_fire.1 == 0 {
                    return false;
                }
                self.barrier_fire.1 -= 1;
                self.barrier_fire.1 != 0
            }
            Status::Barrier(Glyph::Air) => {
                if self.barrier_air.1 == 0 {
                    return false;
                }
                self.barrier_air.1 -= 1;
                self.barrier_air.1 != 0
            }
            Status::Barrier(Glyph::Earth) => {
                if self.barrier_earth.1 == 0 {
                    return false;
                }
                self.barrier_earth.1 -= 1;
                self.barrier_earth.1 != 0
            }
            Status::Barrier(Glyph::Water) => {
                if self.barrier_water.1 == 0 {
                    return false;
                }
                self.barrier_water.1 -= 1;
                self.barrier_water.1 != 0
            }
            Status::Barrier(Glyph::Void) => {
                if self.barrier_void.1 == 0 {
                    return false;
                }
                self.barrier_void.1 -= 1;
                self.barrier_void.1 != 0
            }
            Status::Burning => {
                if self.burning.1 == 0 {
                    return false;
                }
                self.burning.1 -= 1;
                self.burning.1 != 0
            }
            Status::Stunned => {
                if self.stunned.1 == 0 {
                    return false;
                }
                self.stunned.1 -= 1;
                self.stunned.1 != 0
            }
            Status::Submerged => {
                if self.submerged.1 == 0 {
                    return false;
                }
                self.submerged.1 -= 1;
                self.submerged.1 != 0
            }
            Status::Shocked => {
                if self.shocked.1 == 0 {
                    return false;
                }
                self.shocked.1 -= 1;
                self.shocked.1 != 0
            }
            Status::Weakened => {
                if self.weakened.1 == 0 {
                    return false;
                }
                self.weakened.1 -= 1;
                self.weakened.1 != 0
            }
            Status::Raging => {
                if self.raging.1 == 0 {
                    return false;
                }
                self.raging.1 -= 1;
                self.raging.1 != 0
            }
            Status::Hardened => {
                if self.hardened.1 == 0 {
                    return false;
                }
                self.hardened.1 -= 1;
                self.hardened.1 != 0
            }
            Status::Fluid => {
                if self.fluid.1 == 0 {
                    return false;
                }
                self.fluid.1 -= 1;
                self.fluid.1 != 0
            }
            Status::Flying => {
                if self.flying.1 == 0 {
                    return false;
                }
                self.flying.1 -= 1;
                self.flying.1 != 0
            }
        }
    }

    pub fn tick_all(&mut self) {
        if self.barrier_fire.1 > 0 {
            self.barrier_fire.1 -= 1;
        }
        if self.barrier_air.1 > 0 {
            self.barrier_air.1 -= 1;
        }
        if self.barrier_earth.1 > 0 {
            self.barrier_earth.1 -= 1;
        }
        if self.barrier_water.1 > 0 {
            self.barrier_water.1 -= 1;
        }
        if self.barrier_void.1 > 0 {
            self.barrier_void.1 -= 1;
        }
        if self.burning.1 > 0 {
            self.burning.1 -= 1;
        }
        if self.stunned.1 > 0 {
            self.stunned.1 -= 1;
        }
        if self.submerged.1 > 0 {
            self.submerged.1 -= 1;
        }
        if self.shocked.1 > 0 {
            self.shocked.1 -= 1;
        }
        if self.weakened.1 > 0 {
            self.weakened.1 -= 1;
        }
        if self.raging.1 > 0 {
            self.raging.1 -= 1;
        }
        if self.hardened.1 > 0 {
            self.hardened.1 -= 1;
        }
        if self.fluid.1 > 0 {
            self.fluid.1 -= 1;
        }
        if self.flying.1 > 0 {
            self.flying.1 -= 1;
        }
    }
}

impl PartialEq for StatusSet {
    fn eq(&self, other: &Self) -> bool {
        (self.barrier_fire.0 == 0 && other.barrier_fire.0 == 0 || self.barrier_fire.0 > 0 && other.barrier_fire.0 > 0)
        && (self.barrier_air.0 == 0 && other.barrier_air.0 == 0 || self.barrier_air.0 > 0 && other.barrier_air.0 > 0)
        && (self.barrier_earth.0 == 0 && other.barrier_earth.0 == 0 || self.barrier_earth.0 > 0 && other.barrier_earth.0 > 0)
        && (self.barrier_water.0 == 0 && other.barrier_water.0 == 0 || self.barrier_water.0 > 0 && other.barrier_water.0 > 0)
        && (self.barrier_void.0 == 0 && other.barrier_void.0 == 0 || self.barrier_void.0 > 0 && other.barrier_void.0 > 0)
        && (self.burning.0 == 0 && other.burning.0 == 0 || self.burning.0 > 0 && other.burning.0 > 0)
        && (self.stunned.0 == 0 && other.stunned.0 == 0 || self.stunned.0 > 0 && other.stunned.0 > 0)
        && (self.submerged.0 == 0 && other.submerged.0 == 0 || self.submerged.0 > 0 && other.submerged.0 > 0)
        && (self.shocked.0 == 0 && other.shocked.0 == 0 || self.shocked.0 > 0 && other.shocked.0 > 0)
        && (self.weakened.0 == 0 && other.weakened.0 == 0 || self.weakened.0 > 0 && other.weakened.0 > 0)
        && (self.raging.0 == 0 && other.raging.0 == 0 || self.raging.0 > 0 && other.raging.0 > 0)
        && (self.hardened.0 == 0 && other.hardened.0 == 0 || self.hardened.0 > 0 && other.hardened.0 > 0)
        && (self.fluid.0 == 0 && other.fluid.0 == 0 || self.fluid.0 > 0 && other.fluid.0 > 0)
        && (self.flying.0 == 0 && other.flying.0 == 0 || self.flying.0 > 0 && other.flying.0 > 0)
    }
}

impl Eq for StatusSet {}