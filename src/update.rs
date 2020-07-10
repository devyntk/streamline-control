use self_update::update::Release;

pub enum ReleaseStatus {
    NewVersion(Release),
    UpToDate,
}

pub fn fetch_is_new() -> Result<ReleaseStatus, Box<dyn (::std::error::Error)>> {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("bkeeneykid")
        .repo_name("streamline-control")
        .build()?
        .fetch()?;

    if releases.is_empty() {
        return Ok(ReleaseStatus::UpToDate);
    }

    let latest_release = &releases[0];

    let is_new =
        self_update::version::bump_is_greater(cargo_crate_version!(), &latest_release.version)?;

    if is_new {
        Ok(ReleaseStatus::NewVersion(latest_release.clone()))
    } else {
        Ok(ReleaseStatus::UpToDate)
    }
}

pub fn do_update() -> Result<(), Box<dyn (::std::error::Error)>> {
    self_update::backends::github::Update::configure()
        .repo_owner("bkeeneykid")
        .repo_name("streamline-control")
        .build()?
        .update()?;
    Ok(())
}
