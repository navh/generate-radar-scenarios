use indicatif::ProgressIterator;
use rand::Rng;
use serde::Serialize;
use serde_json::Result;
use std::io;
use uuid::Uuid;

const VERSION: u32 = 0;
const ABOUT: &str = "Standardized Radar Task Selection Format for Radar Resource Management";
const LONG_ABOUT: &str = "Currently, radar scheduling algorithms are being evaluated on randomly generated tasks, this means that interesting results are at times ephemeral, it is difficult to manually create and evaluate tricky sets of tasks to evaluate algorithms are behaving as expected, comparing performance cross different implementations is difficult, and results are difficult to reproduce. This document contains everything needed to define a radar scenario. It is a ScenarioPack which contains many Scenarios, each Scenario is created with the same random seeds, and contains many Tasks.";

#[derive(Serialize)]
struct ScenarioPack {
    pack_id: Uuid,
    version: u32,
    about: String,
    long_about: String,
    scenario_params: ScenarioParams,
    scenarios: Vec<Scenario>,
}

#[derive(Serialize)]
pub struct ScenarioParams {
    pub scenario_count: u32,
    pub task_count: u32,
    pub start_time: f32,
    pub end_time: f32,
    pub min_task_length: f32,
    pub max_task_length: f32,
    pub min_tardiness_cost: f32,
    pub max_tardiness_cost: f32,
    pub min_drop_cost: f32,
    pub max_drop_cost: f32,
    pub earliest_time_is_no_cost: bool,
}

#[derive(Serialize)]
struct Scenario {
    scenario_id: u32,
    tasks: Vec<Task>,
}

// Tasks have: task_id, earliest_time, no_cost_time, latest_time, task_length, tardiness_cost, drop_cost
#[derive(Serialize)]
struct Task {
    task_id: u32,
    length: f32,
    earliest_time: f32,
    no_cost_time: f32,
    latest_time: f32,
    tardiness_cost: f32,
    drop_cost: f32,
}

pub fn run(params: ScenarioParams) -> Result<()> {
    let scenarios: Vec<Scenario> = (0..params.scenario_count)
        .progress()
        .map(|scenario_id| generate_scenario(&params, scenario_id))
        .collect();

    let pack = ScenarioPack {
        version: VERSION,
        about: ABOUT.to_string(),
        long_about: LONG_ABOUT.to_string(),
        pack_id: Uuid::new_v4(),
        scenario_params: params,
        scenarios,
    };

    serde_json::to_writer(io::stdout(), &pack)
}

fn generate_scenario(params: &ScenarioParams, scenario_id: u32) -> Scenario {
    let tasks: Vec<Task> = (0..params.task_count)
        .map(|task_id| generate_task(&params, task_id))
        .collect();

    Scenario { scenario_id, tasks }
}

fn generate_task(params: &ScenarioParams, task_id: u32) -> Task {
    let mut rng = rand::thread_rng();

    let length = rng.gen_range(params.min_task_length..=params.max_task_length);

    let latest_time_end_time_could_be = params.end_time - length;
    let no_cost_time = rng.gen_range(params.start_time..=latest_time_end_time_could_be);

    let earliest_time;
    if params.earliest_time_is_no_cost {
        earliest_time = no_cost_time;
    } else {
        earliest_time = rng.gen_range(params.start_time..=no_cost_time);
    };

    let latest_time = rng.gen_range(no_cost_time..=params.end_time);

    let tardiness_cost = rng.gen_range(params.min_tardiness_cost..=params.max_tardiness_cost);
    let drop_cost = rng.gen_range(params.min_drop_cost..=params.max_drop_cost);

    Task {
        task_id,
        no_cost_time,
        earliest_time,
        latest_time,
        length,
        tardiness_cost,
        drop_cost,
    }
}
