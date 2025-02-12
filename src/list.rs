/*
 * Copyright (c) 2019 Jonathan Perkin <jonathan@perkin.org.uk>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * list.rs - handle commands that list packages (avail, list)
 */

use crate::config;
use crate::pmdb::PMDB;

#[derive(Debug)]
pub struct ListPackage {
    pub pkgname: String,
    pub comment: String,
}

pub fn avail(
    cfg: &config::Config,
    db: &mut PMDB,
) -> Result<(), Box<dyn std::error::Error>> {
    let pkgs = db.get_remote_pkgs_by_prefix(cfg.prefix())?;
    if pkgs.is_empty() {
        eprintln!("No packages available for prefix={}", cfg.prefix());
        std::process::exit(1);
    }
    for pkg in pkgs {
        println!("{:20} {}", pkg.pkgname(), pkg.comment());
    }
    Ok(())
}

pub fn list(
    cfg: &config::Config,
    db: &mut PMDB,
) -> Result<(), Box<dyn std::error::Error>> {
    let pkgs = db.get_local_pkgs_by_prefix(cfg.prefix())?;
    if pkgs.is_empty() {
        eprintln!("No packages recorded under {}", cfg.prefix());
        std::process::exit(1);
    }
    for pkg in pkgs {
        println!("{:20} {}", pkg.pkgname(), pkg.comment());
    }
    Ok(())
}

impl ListPackage {
    pub fn pkgname(&self) -> &String {
        &self.pkgname
    }
    pub fn comment(&self) -> &String {
        &self.comment
    }
}
