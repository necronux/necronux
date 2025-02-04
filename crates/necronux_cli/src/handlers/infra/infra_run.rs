// ==----------------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2024-2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==----------------------------------------------------------------== //

use crate::{
    commands::lvl_1::infra::{Hosts, InfraRunArgs, SetupExtraFlags},
    handlers::infra::run_trinity_heliolens_setup::{
        trinity_heliolens_part_one, trinity_heliolens_part_two,
    },
};
use color_eyre::eyre::Result;
use log::{debug, info};

pub fn handle_infra_run(infra_run_arg: &InfraRunArgs) -> Result<()> {
    match infra_run_arg.host {
        Hosts::TrinityHeliolens => {
            info!("Host: heliolens was provided");

            if let Some(extra_flag) = infra_run_arg.extra_flag {
                match extra_flag {
                    SetupExtraFlags::PartOne => {
                        info!("Extra flag: part-one was provided");
                        info!("Setting up infrastructure for Trinity-Heliolens");
                        debug!("Running Trinity-Heliolens setup part one script");
                        trinity_heliolens_part_one()?;
                    }
                    SetupExtraFlags::PartTwo => {
                        info!("Extra flag: part-two was provided");
                        info!("Setting up infrastructure for Trinity-Heliolens");
                        debug!("Running Trinity-Heliolens setup part two script");
                        trinity_heliolens_part_two()?;
                    }
                }
            }
        }
        Hosts::TrinityInfinity => {
            info!("Host: infinity was provided");
            info!("Setting up infrastructure for Infinity");
            info!("Infinity setup not implemented yet");
        }
    }

    Ok(())
}
