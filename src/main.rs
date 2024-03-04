use std::{env::current_dir, io};

use cargo::core::resolver::CliFeatures;
use cargo::ops::{cargo_config, resolve_ws, CompileOptions};
use cargo::util::OptVersionReq;
use cargo::Config;
use cargo::{core::Workspace, util::important_paths::find_root_manifest_for_wd};

use semver::VersionReq;

fn main() -> io::Result<()> {
    let manifest_path = find_root_manifest_for_wd(&current_dir()?).unwrap();

    let mut cargo_config = Config::default().unwrap();

    cargo_config
        .configure(
            0,
            false,
            None,
            false,
            true,
            false,
            &None,
            &["minimal-versions".to_owned()],
            &[],
        )
        .unwrap();

    let mut workspace = Workspace::new(&manifest_path, &cargo_config).unwrap();

    for member in workspace.members_mut() {
        let new_summary = member
            .manifest_mut()
            .summary_mut()
            .to_owned()
            .map_dependencies(|mut dep| {
                // dep.set_version_req(OptVersionReq::Req(VersionReq::parse(">2.0.0").unwrap()));
                dep
            });

        *member.manifest_mut().summary_mut() = new_summary;
    }

    if workspace.config().cli_unstable().minimal_versions {
        println!("true")
    }

    let (pakage_list, resolve) = resolve_ws(&workspace).unwrap();

    println!("{:#?}", resolve);

    let mut check_compile_options = CompileOptions::new(
        &cargo_config,
        cargo::util::command_prelude::CompileMode::Check { test: false },
    ).unwrap();

    check_compile_options.cli_features = CliFeatures::new_all(true);


    // cargo::ops::compile(
    //     &workspace,
    // &CompileOptions::new(
    //     &default_cargo_config,
    //     cargo::util::command_prelude::CompileMode::Check { test: false },
    // )
    //     .unwrap(),
    // )
    // .unwrap();

    Ok(())
}
