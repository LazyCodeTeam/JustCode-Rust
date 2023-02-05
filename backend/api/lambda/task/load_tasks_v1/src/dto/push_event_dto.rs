use git_domain::model::git_hook_event::GitHookEvent;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct PushEventDto {
    #[serde(rename = "ref")]
    pub reference: String,
    // ... tons of other fields that we don't care about
}

impl From<PushEventDto> for GitHookEvent {
    fn from(dto: PushEventDto) -> GitHookEvent {
        GitHookEvent::Push {
            reference: dto.reference,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn map_to_push_hook_event() {
        let dto = PushEventDto {
            reference: "refs/heads/master".to_owned(),
        };

        let event = GitHookEvent::from(dto);

        assert_eq!(
            event,
            GitHookEvent::Push {
                reference: "refs/heads/master".to_owned()
            }
        );
    }
}
