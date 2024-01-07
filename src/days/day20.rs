use std::str;

use axum::body::Bytes;
use axum::http::StatusCode;
use axum::routing::post;
use axum::Router;
use bytes::Buf;
use git2::{self, Oid, Repository, TreeEntry};
use tar::Archive;

pub fn get_day_20_router() -> Router {
    Router::new()
        .route("/archive_files", post(archive_files))
        .route("/archive_files_size", post(archive_files_size))
        .route("/cookie", post(cookie))
}

async fn archive_files(body: Bytes) -> (StatusCode, String) {
    (
        StatusCode::OK,
        Archive::new(body.reader())
            .entries()
            .unwrap()
            .count()
            .to_string(),
    )
}

async fn archive_files_size(body: Bytes) -> (StatusCode, String) {
    (
        StatusCode::OK,
        Archive::new(body.reader())
            .entries()
            .unwrap()
            .map(|file| file.unwrap().size())
            .sum::<u64>()
            .to_string(),
    )
}

async fn cookie(body: Bytes) -> (StatusCode, String) {
    let tmp_dir = tempfile::tempdir().unwrap();
    Archive::new(body.reader()).unpack(&tmp_dir).unwrap();
    let repository = Repository::open(tmp_dir.path()).unwrap();

    let (committer_name, commit_id) = find_commit(&repository, "christmas", "santa.txt", "COOKIE");

    (StatusCode::OK, format!("{} {}", committer_name, commit_id))
}

fn find_commit(
    repository: &Repository,
    branch_name: &str,
    file_name: &str,
    text_to_find: &str,
) -> (String, Oid) {
    let branch = repository
        .find_branch(branch_name, git2::BranchType::Local)
        .unwrap();
    let head_commit = branch.get().peel_to_commit().unwrap();
    let mut commit = head_commit;
    while commit.parent_count() > 0 {
        let mut find_cookie = false;
        commit
            .tree()
            .unwrap()
            .walk(git2::TreeWalkMode::PreOrder, |_, entry| {
                if right_file(repository, file_name, text_to_find, entry) {
                    find_cookie = true;
                    git2::TreeWalkResult::Abort
                } else {
                    git2::TreeWalkResult::Ok
                }
            })
            .unwrap();
        if find_cookie {
            break;
        }

        commit = commit.parent(0).unwrap();
    }
    (
        commit.clone().author().name().unwrap().to_string(),
        commit.id(),
    )
}

fn right_file(
    repository: &Repository,
    file_name: &str,
    text_to_find: &str,
    entry: &TreeEntry,
) -> bool {
    entry.name() == Some(file_name)
        && str::from_utf8(
            entry
                .to_object(repository)
                .unwrap()
                .as_blob()
                .unwrap()
                .content(),
        )
        .unwrap()
        .contains(text_to_find)
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;

    use super::*;

    #[tokio::test]
    async fn task1a() {
        let app = get_day_20_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        let file_bytes = Bytes::from(include_bytes!("../../assets/northpole20231220.tar").to_vec());

        // Send the request.
        let response = server.post("/archive_files").bytes(file_bytes).await;

        response.assert_status(StatusCode::OK);

        response.assert_text(6.to_string());
    }

    #[tokio::test]
    async fn task1b() {
        let app = get_day_20_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        let file_bytes = Bytes::from(include_bytes!("../../assets/northpole20231220.tar").to_vec());

        // Send the request.
        let response = server.post("/archive_files_size").bytes(file_bytes).await;

        response.assert_status(StatusCode::OK);

        response.assert_text(1196282.to_string());
    }

    #[tokio::test]
    async fn task2() {
        let app = get_day_20_router();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        let file_bytes = Bytes::from(include_bytes!("../../assets/cookiejar.tar").to_vec());

        // Send the request.
        let response = server.post("/cookie").bytes(file_bytes).await;

        response.assert_status(StatusCode::OK);

        response.assert_text("Grinch 71dfab551a1958b35b7436c54b7455dcec99a12c");
    }
}
