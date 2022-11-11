use super::*;

pub struct AudioContext {
    context: web_sys::AudioContext,
}

impl AudioContext {
    pub(crate) fn new() -> Self {
        Self {
            context: web_sys::AudioContext::new().expect("Failed to initialize audio context"),
        }
    }

    pub fn set_listener_position(&self, pos: Vec3<f64>) {
        self.context.listener().set_position(pos.x, pos.y, pos.z);
    }

    pub fn set_listener_orientation(&self, forward: Vec3<f64>, up: Vec3<f64>) {
        self.context
            .listener()
            .set_orientation(forward.x, forward.y, forward.z, up.x, up.y, up.z);
    }
}

enum SpatialState {
    NotSpatial(web_sys::MediaElementAudioSourceNode),
    Spatial(web_sys::PannerNode),
}

pub struct Sound {
    geng: Geng,
    inner: web_sys::HtmlAudioElement,
    pub looped: bool,
}

impl Sound {
    pub(crate) fn new(geng: &Geng, audio: web_sys::HtmlAudioElement) -> Self {
        Self {
            geng: geng.clone(),
            inner: audio,
            looped: false,
        }
    }
    pub fn effect(&self) -> SoundEffect {
        let effect = self
            .inner
            .clone_node()
            .unwrap()
            .dyn_into::<web_sys::HtmlAudioElement>()
            .unwrap();
        let audio_node = self
            .geng
            .inner
            .audio
            .context
            .create_media_element_source(&effect)
            .unwrap();
        audio_node
            .connect_with_audio_node(&self.geng.inner.audio.context.destination())
            .unwrap();
        effect.set_loop(self.looped);
        SoundEffect {
            geng: self.geng.clone(),
            inner: effect,
            spatial_state: SpatialState::NotSpatial(audio_node),
        }
    }
    pub fn play(&self) -> SoundEffect {
        let mut effect = self.effect();
        effect.play();
        effect
    }
}

pub struct SoundEffect {
    geng: Geng,
    inner: web_sys::HtmlAudioElement,
    spatial_state: SpatialState,
}

impl SoundEffect {
    pub fn set_volume(&mut self, volume: f64) {
        self.inner.set_volume(volume);
    }
    pub fn play(&mut self) {
        let _ = self.inner.play().unwrap();
    }
    pub fn stop(mut self) {
        self.pause();
    }
    pub fn pause(&mut self) {
        self.inner.pause().unwrap();
    }
    pub fn set_position(&mut self, position: Vec3<f64>) {
        let panner_node = self.make_spatial();
        panner_node.set_position(position.x, position.y, position.z);
    }
    pub fn set_ref_distance(&mut self, distance: f64) {
        let panner_node = self.make_spatial();
        panner_node.set_ref_distance(distance);
    }
    pub fn set_max_distance(&mut self, max_distance: f64) {
        let panner_node = self.make_spatial();
        panner_node.set_max_distance(max_distance);
    }
    fn make_spatial(&mut self) -> &web_sys::PannerNode {
        if let SpatialState::NotSpatial(audio_node) = &self.spatial_state {
            let panner_node = web_sys::PannerNode::new(&self.geng.inner.audio.context).unwrap();
            panner_node.set_distance_model(web_sys::DistanceModelType::Linear);
            audio_node.disconnect().unwrap();
            audio_node
                .connect_with_audio_node(&panner_node)
                .unwrap()
                .connect_with_audio_node(&self.geng.inner.audio.context.destination())
                .unwrap();
            self.spatial_state = SpatialState::Spatial(panner_node);
        }
        let SpatialState::Spatial(panner_node) = &self.spatial_state else {
            unreachable!()
        };
        panner_node
    }
}

impl Drop for SoundEffect {
    fn drop(&mut self) {}
}
