use askama::Template;
use warp::{reply::html, Reply};

use crate::{PackageVersion, Project, WebResult};

#[derive(Template)]
#[template(path = "projects.html")]
struct ProjectList<'p> {
    projects: &'p Vec<Project>,
}

#[derive(Template)]
#[template(path = "versions.html")]
struct VersionList<'p> {
    versions: &'p Vec<PackageVersion>,
}

pub async fn project_list_handler() -> WebResult<impl Reply> {
    let projects = vec![Project {
        url: String::from("/dummy/"),
        name: String::from("dummy"),
    }];

    let template = ProjectList {
        projects: &projects,
    };
    let res = template.render().unwrap();
    Ok(html(res))
}

pub async fn version_list_handler() -> WebResult<impl Reply> {
    let versions = vec![
        PackageVersion {
            download_url: String::from("../packages/dummy-0.1.tar.gz"),
            package_name: String::from("dummy"),
            name: String::from("0.1"),
        },
        PackageVersion {
            download_url: String::from("../packages/dummy-0.2.tar.gz"),
            package_name: String::from("dummy"),
            name: String::from("0.2"),
        },
    ];

    let template = VersionList {
        versions: &versions,
    };
    let res = template.render().unwrap();
    Ok(html(res))
}
