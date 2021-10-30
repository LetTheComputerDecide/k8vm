use anyhow::{anyhow, Result};
use octocrab::{
    models::{repos::Release},
    Octocrab, Page,
};

use futures::future::try_join_all;

async fn get_release_page(octocrab: &Octocrab, page_size: u8, page: u32) -> Result<Page<Release>> {
    octocrab
        .repos("kubernetes", "kubernetes")
        .releases()
        .list()
        .per_page(page_size)
        .page(page)
        .send()
        .await
        .map_err(|err| anyhow!(err))
}

async fn get_all_releases(octocrab: &Octocrab) -> Result<Vec<Release>> {
    let page_size = 100;
    let mut page = get_release_page(octocrab, page_size, 1).await?;

    let pages = try_join_all(
        (2..=page.number_of_pages().unwrap()).map(|page| get_release_page(octocrab, 100, page)),
    )
    .await?;

    let mut releases = vec![];
    releases.append(&mut page.items);
    releases.append(
        &mut pages
            .into_iter()
            .flat_map(|page| page.items)
            .collect::<Vec<_>>(),
    );

    Ok(releases)
}

pub async fn list_remote() -> Result<()> {
    let octocrab = octocrab::instance();

    let mut tag_names = get_all_releases(&octocrab)
        .await?
        .into_iter()
        .map(|r| r.tag_name)
        .collect::<Vec<_>>();

    tag_names.sort();
    tag_names.into_iter().rev().for_each(|tn| println!("{}", tn));

    Ok(())
}
