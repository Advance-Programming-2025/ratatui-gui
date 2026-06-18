//! Side-effect commands emitted by UI controller.
//! Executed by game loop against orchestrator and model.

use common_game::components::resource::{ResourceType};

use crate::game_state::GameState;

/// Side-effect requests emitted by controller.
#[derive(Debug, Clone)]
pub(crate) enum Command {
    SetGameState(GameState),
    StartGame,
    StopAll,
    RestartAll,
    StopExplorerAI {
        explorer_id: u32,
    },
    ToggleLog,
    AdjustCustomPlanets(i32),
    ToggleGenerationMode,
    QueueAsteroid {
        planet_id: u32,
    },
    QueueSunray {
        planet_id: u32,
    },
    MoveExplorer {
        explorer_id: u32,
        planet_id: u32,
    },
    GenerateResource {
        explorer_id: u32,
        resource: ResourceType,
    },
}
