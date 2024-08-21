use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn e2e() -> FnResult<String> {
    let stdout = dag()
        .mise()?
        .with_exec(vec![
            "mise install && cd src/client && mise x -- bun install && mise x -- bun run test:e2e",
        ])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn server_e2e() -> FnResult<String> {
    let stdout = dag()
        .mise()?
        .with_exec(vec![
            "mise install && cd src/server && cp ormconfig.ci.json ormconfig.json && mise x -- bun install && mise x -- bun run migrate:up && mise x -- bun run test:e2e",
        ])?
        .stdout()?;
    Ok(stdout)
}
