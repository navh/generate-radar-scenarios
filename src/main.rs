use clap::Parser;
use std::process;

use generate_radar_scenarios::ScenarioParams;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Args {
    /// Number of tasks to generate
    #[arg(short, long, default_value_t = 5)]
    tasks: u32,

    /// Number of scenarios to generate
    #[arg(short, long, default_value_t = 100)]
    scenarios: u32,

    /// Beginning time of scenario
    #[arg(long, default_value_t = 0.0)]
    start_time: f32,

    /// Minimum length of a task
    #[arg(long, default_value_t = 0.0)]
    min_task_length: f32,

    /// Maximum length of a task
    #[arg(long, default_value_t = 0.4)]
    max_task_length: f32,

    /// Minimum tardiness coefficient
    #[arg(long, default_value_t = 0.0)]
    min_tardiness_cost: f32,

    /// Maximum length of a task
    #[arg(long, default_value_t = 1.0)]
    max_tardiness_cost: f32,

    /// Minimum drop cost
    #[arg(long, default_value_t = 0.0)]
    min_drop_cost: f32,

    /// Maximum drop cost
    #[arg(long, default_value_t = 1.0)]
    max_drop_cost: f32,

    /// Ending time of scenario
    #[arg(long, default_value_t = 1.0)]
    end_time: f32,

    /// set no_cost_time to earliest_time, otherwise, no_cost_time is random point between earliest_time and latest_time
    #[arg(long, action)]
    earliest_time_is_no_cost: bool,
}

fn main() {
    let args = Args::parse();

    let params = ScenarioParams {
        scenario_count: args.scenarios,
        task_count: args.tasks,
        start_time: args.start_time,
        end_time: args.end_time,
        min_task_length: args.min_task_length,
        max_task_length: args.max_task_length,
        min_tardiness_cost: args.min_tardiness_cost,
        max_tardiness_cost: args.max_tardiness_cost,
        min_drop_cost: args.min_drop_cost,
        max_drop_cost: args.max_drop_cost,
        earliest_time_is_no_cost: args.earliest_time_is_no_cost,
    };

    if let Err(e) = generate_radar_scenarios::run(params) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
