use bevy::{app::MainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};

use crate::balls::BallHitWallEvent;

#[derive(ScheduleLabel, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ScoreAddPhase;
#[derive(ScheduleLabel, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ScoreMultiplyPhase;
#[derive(ScheduleLabel, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ScoreCommitPhase;

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(ScoreAddPhase);
        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_after(PostUpdate, ScoreAddPhase);
        app.init_schedule(ScoreMultiplyPhase);
        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_after(ScoreAddPhase, ScoreMultiplyPhase);
        app.init_schedule(ScoreCommitPhase);
        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_after(ScoreMultiplyPhase, ScoreCommitPhase);
        app.add_observer(add_score_on_ball_hit_wall);
        app.add_systems(ScoreCommitPhase, commit_scores);
    }
}

// Component to track score added this round
// At the end of the round we will trigger a score event with the total score
// Ball properties operate on the score component in phases
// Add
// Multiply

#[derive(Component, Default)]
pub struct Score {
    pub score: u32,
}

#[derive(Event)]
pub struct IncreaseScoreEvent {
    pub amount: u32,
}

fn add_score_on_ball_hit_wall(trigger: Trigger<BallHitWallEvent>, mut commands: Commands) {
    // Insert score into ball
    commands.entity(trigger.target()).insert(Score { score: 1 });
    println!("Added score to ball: {:?}", trigger.target());
}

fn commit_scores(mut commands: Commands, score_query: Query<(Entity, &Score)>) {
    let total_score: u32 = score_query.iter().map(|(_entity, score)| score.score).sum();
    commands.trigger(IncreaseScoreEvent {
        amount: total_score,
    });

    for (entity, _) in score_query.iter() {
        commands.entity(entity).remove::<Score>();
        println!("Committed score for ball: {:?}", entity);
    }
}
