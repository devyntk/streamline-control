use self_update::update::{Release};

pub enum UpdateStatus{
    NewVersion(Release),
    UpToDate
}

pub fn fetch_is_new<'a>() -> Result<UpdateStatus, Box<dyn (::std::error::Error)>>{
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("bkeeneykid")
        .repo_name("streamline-control")
        .build()?
        .fetch()?;
    println!("found releases:");
    println!("{:#?}\n", releases);

    if releases.len() == 0 {
        return Ok(UpdateStatus::UpToDate)
    }

    let latest_release = &releases[0];

    let is_new = self_update::version::bump_is_greater(cargo_crate_version!(),
                                                       &latest_release.version)?;

    if is_new {
        Ok(UpdateStatus::NewVersion(latest_release.clone()))
    } else {
        Ok(UpdateStatus::UpToDate)
    }
}