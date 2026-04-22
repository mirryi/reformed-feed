mod config;

use anyhow::{bail, Context, Result};
use chrono::Utc;
use clap::Parser;
use reformed_feed::feed::document::{Document, IntoFeedEntry};
use reformed_feed::feed::generate::{self, store_entry, FeedConfig};
use reformed_feed::feed::schedule::Schedule;
use reformed_feed::feed::state::{JsonFileStore, PersistedState, StateStore};
use reformed_creeds::registry::{self, Shape, Source};
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "RSS feed generator for Reformed confessions and catechisms")]
struct Cli {
    /// Path to the TOML config file
    config: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config_path = cli.config.display().to_string();
    let config = config::Config::load(&config_path)
        .with_context(|| format!("Failed to load config from {}", config_path))?;

    run(&config)
}

fn run(config: &config::Config) -> Result<()> {
    let mut all_doc_ids: Vec<String> = Vec::new();
    let mut all_doc_lengths: Vec<usize> = Vec::new();
    let mut all_items: Vec<Vec<Box<dyn IntoFeedEntry>>> = Vec::new();

    for doc_entry in &config.documents {
        let reg = registry::lookup(&doc_entry.id)
            .ok_or_else(|| anyhow::anyhow!("Unknown document ID: {}", doc_entry.id))?;

        let file_path = resolve_file_path(reg, doc_entry, config);
        let data = fs::read(&file_path)
            .with_context(|| format!("Failed to read {}", file_path))?;

        let parse_name = doc_entry.parse.as_deref().unwrap_or(reg.default_parse);
        let items = parse_and_collect(reg, parse_name, &data)?;

        all_doc_ids.push(reg.id.to_string());
        all_doc_lengths.push(items.len());
        all_items.push(items);
    }

    let doc_id_refs: Vec<&str> = all_doc_ids.iter().map(|s| s.as_str()).collect();
    let now = Utc::now();

    match config.schedule.preset.as_str() {
        "daily-sequential" => {
            let schedule = reformed_feed::schedules::presets::daily_sequential(config.schedule.hour.unwrap_or(8));
            run_with_schedule(&schedule, config, &doc_id_refs, &all_doc_lengths, &all_items, now)
        }
        "daily-round-robin" => {
            let schedule = reformed_feed::schedules::presets::daily_round_robin(config.schedule.hour.unwrap_or(8));
            run_with_schedule(&schedule, config, &doc_id_refs, &all_doc_lengths, &all_items, now)
        }
        "weighted-daily" => {
            let weights = config.schedule.weights.clone().unwrap_or_default();
            let schedule =
                reformed_feed::schedules::presets::weighted_daily(config.schedule.hour.unwrap_or(8), weights);
            run_with_schedule(&schedule, config, &doc_id_refs, &all_doc_lengths, &all_items, now)
        }
        "frequent" => {
            let schedule =
                reformed_feed::schedules::presets::frequent(config.schedule.interval_hours.unwrap_or(8));
            run_with_schedule(&schedule, config, &doc_id_refs, &all_doc_lengths, &all_items, now)
        }
        "weekly-digest" => {
            let schedule =
                reformed_feed::schedules::presets::weekly_digest(config.schedule.items_per_week.unwrap_or(5));
            run_with_schedule(&schedule, config, &doc_id_refs, &all_doc_lengths, &all_items, now)
        }
        other => bail!("Unknown schedule preset: {}", other),
    }
}

fn run_with_schedule<S: Schedule>(
    schedule: &S,
    config: &config::Config,
    doc_ids: &[&str],
    doc_lengths: &[usize],
    all_items: &[Vec<Box<dyn IntoFeedEntry>>],
    now: chrono::DateTime<Utc>,
) -> Result<()> {
    let store = JsonFileStore::new(&config.state.path);

    let mut persisted: PersistedState<S::State> = match store.load()? {
        Some(state) => state,
        None => PersistedState {
            schedule_state: schedule.init_state(doc_ids, doc_lengths),
            recent_items: VecDeque::new(),
        },
    };

    if let Some(item_ref) = schedule.next_if_due(&persisted.schedule_state, now) {
        let doc_idx = doc_ids
            .iter()
            .position(|&id| id == item_ref.doc_id)
            .ok_or_else(|| anyhow::anyhow!("Doc not found: {}", item_ref.doc_id))?;
        let items = &all_items[doc_idx];

        if item_ref.item_index < items.len() {
            let entry = store_entry(items[item_ref.item_index].as_ref());
            persisted.recent_items.push_back(entry);

            while persisted.recent_items.len() > config.feed.max_items {
                persisted.recent_items.pop_front();
            }

            schedule.advance(&mut persisted.schedule_state, &item_ref, now);
            println!("Emitted: {} #{}", item_ref.doc_id, item_ref.item_index);
        }
    } else {
        println!("Not yet time to emit.");
    }

    let feed_config = FeedConfig {
        title: config.feed.title.clone(),
        description: config.feed.description.clone(),
        link: config.feed.link.clone(),
        max_items: config.feed.max_items,
    };
    let items_slice: Vec<_> = persisted.recent_items.iter().cloned().collect();
    let xml = generate::generate_feed(&items_slice, &feed_config)?;
    fs::write(&config.feed.output, xml)?;

    store.save(&persisted)?;

    Ok(())
}

fn resolve_file_path(
    reg: &registry::RegistryEntry,
    doc_entry: &config::DocumentEntry,
    config: &config::Config,
) -> String {
    if let Some(ref file) = doc_entry.file {
        return file.clone();
    }
    match reg.source {
        Source::CreedsJson => {
            format!("{}/creeds/{}", config.sources.creeds_json, reg.filename)
        }
        Source::Compendium => {
            format!("{}/data/{}", config.sources.compendium, reg.filename)
        }
    }
}

fn parse_and_collect(
    reg: &registry::RegistryEntry,
    parse_name: &str,
    data: &[u8],
) -> Result<Vec<Box<dyn IntoFeedEntry>>> {
    use reformed_feed::bridge::parse;
    use reformed_feed::feed::document::ParseStrategy as _;

    let id = reg.id.to_string();
    let title = reg.title.to_string();

    match (reg.shape, parse_name) {
        (Shape::Catechism, "by-question") => {
            let parser = parse::ByQuestion { doc_id: id, doc_title: title };
            let doc = parser.parse(data)?;
            Ok(doc.items().into_iter().map(|i| Box::new(i) as Box<dyn IntoFeedEntry>).collect())
        }
        (Shape::Confession, "by-section") => {
            let parser = parse::BySection { doc_id: id, doc_title: title };
            let doc = parser.parse(data)?;
            Ok(doc.items().into_iter().map(|i| Box::new(i) as Box<dyn IntoFeedEntry>).collect())
        }
        (Shape::Confession, "by-chapter") => {
            let parser = parse::ByChapter { doc_id: id, doc_title: title };
            let doc = parser.parse(data)?;
            Ok(doc.items().into_iter().map(|i| Box::new(i) as Box<dyn IntoFeedEntry>).collect())
        }
        (Shape::Canon, "by-article") => {
            let parser = parse::CanonByArticle { doc_id: id, doc_title: title };
            let doc = parser.parse(data)?;
            Ok(doc.items().into_iter().map(|i| Box::new(i) as Box<dyn IntoFeedEntry>).collect())
        }
        (Shape::Creed, "whole-document") => {
            let parser = parse::WholeCreed { doc_id: id, doc_title: title };
            let doc = parser.parse(data)?;
            Ok(doc.items().into_iter().map(|i| Box::new(i) as Box<dyn IntoFeedEntry>).collect())
        }
        (Shape::Articles, "by-article") => {
            let parser = parse::ArticlesByArticle { doc_id: id, doc_title: title };
            let doc = parser.parse(data)?;
            Ok(doc.items().into_iter().map(|i| Box::new(i) as Box<dyn IntoFeedEntry>).collect())
        }
        (Shape::Theses, "by-thesis") => {
            let parser = parse::ByThesis { doc_id: id, doc_title: title };
            let doc = parser.parse(data)?;
            Ok(doc.items().into_iter().map(|i| Box::new(i) as Box<dyn IntoFeedEntry>).collect())
        }
        _ => bail!(
            "Unsupported parse strategy '{}' for shape {:?}",
            parse_name,
            reg.shape
        ),
    }
}
