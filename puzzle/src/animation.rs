use std::{collections::HashMap, hash::Hash, time::Duration};

use bevy::prelude::*;

#[derive(Default)]
pub struct AnimationData {
    start_index: usize,
    len: usize,
    timer: Timer,
}

impl AnimationData {
    pub fn from_frames(start_index : usize, len : usize) -> Self {
        Self { start_index, len, timer: Timer::from_seconds(1.0, TimerMode::Once) }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.timer.set_duration(duration);
        self
    }

    pub fn with_dur_sec(self, duration: f32) -> Self {
        self.with_duration(Duration::from_secs_f32(duration))
    }

    pub fn repeating(mut self) -> Self {
        self.timer.set_mode(TimerMode::Repeating);
        self
    }

    pub fn once(mut self) -> Self {
        self.timer.set_mode(TimerMode::Once);
        self
    }
}

pub trait FrameAnimation : Component {
    fn advance(&mut self, time: Duration);
    fn current_frame(&self) -> usize;
}

#[derive(Component, Default)]
pub struct SimpleAnimation {
    data: AnimationData,
    current: usize,
}

impl SimpleAnimation {
    pub fn from_data(data: AnimationData) -> Self {
        Self { data, current : 0 }
    }
}

impl FrameAnimation for SimpleAnimation {
    fn advance(&mut self, time: Duration) {
        let anim = &mut self.data;
        
        anim.timer.tick(time);
        let frames_to_advance = anim.timer.times_finished_this_tick();
        self.current = (self.current + frames_to_advance as usize) % anim.len;
    }

    fn current_frame(&self) -> usize {
        self.current + self.data.start_index
    }
}

#[derive(Component)]
pub struct AnimationStateMachine<State: Hash + Eq + Sync + Send + 'static> {
    pub map: HashMap<State, AnimationData>,
    pub current_state: State,
    current_frame: usize
}

impl<State: Hash + Eq + Sync + Send + 'static> AnimationStateMachine<State> {
    pub fn from_data(data: impl IntoIterator<Item=(State, AnimationData)>, start_state: State) -> Self {
        let mut map = HashMap::new();

        for (state, anim_data) in data {
            map.insert(state, anim_data);
        }

        Self { map, current_state : start_state, current_frame: 0 }
    }

    pub fn switch_state(&mut self, new_state: State) {
        if new_state != self.current_state  {
            self.current_state = new_state;
            self.current_frame = 0;
        }
    }
}

impl<State: Hash + Eq + Sync + Send + 'static> FrameAnimation for AnimationStateMachine<State> {
    fn advance(&mut self, time: Duration) {
        let anim = &mut self.map.get_mut(&self.current_state).unwrap();
        
        anim.timer.tick(time);
        let frames_to_advance = anim.timer.times_finished_this_tick();
        self.current_frame = (self.current_frame + frames_to_advance as usize) % anim.len;
    }

    fn current_frame(&self) -> usize {
        self.current_frame + self.map.get(&self.current_state).unwrap().start_index
    }
}

pub fn advance_animation_system<A : Component + FrameAnimation>(mut query : Query<&mut A>, time: Res<Time>) {
    for mut anim in query.iter_mut() {
        anim.advance(time.delta());
    }
}

pub fn apply_animation_system<A : Component + FrameAnimation>(mut query : Query<(&A, &mut TextureAtlasSprite)>) {
    for (anim, mut sprite) in query.iter_mut() {
        sprite.index = anim.current_frame();
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(advance_animation_system::<SimpleAnimation>);
    }
}