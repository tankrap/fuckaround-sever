use chrono::{DateTime, Utc};
use convert_case::ccase;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
pub trait StructToString
{
    fn get_db_name() -> String;
}

#[macro_export]
macro_rules! model {
    (
        $name:ident, {
            $( pub $field:ident : $fty:ty ),* $(,)?
        }
    ) => {
        #[derive(Serialize, Deserialize, FromRow, Debug, Clone, Eq, PartialEq)]
        pub struct $name  {
            $( pub(crate) $field: $fty ),*
        }

        impl StructToString for $name {
            fn get_db_name() -> String {
                return ccase!(snake, stringify!($name))
            }
        }
    };
}
model!(Config, {
    pub key: String,
    pub value: serde_json::Value
});

model!(AccountCredentials, {
    pub id: String,
    pub related_account: String,
    pub public_key: String,
    pub credential_added_when: DateTime<Utc>,
    pub last_used: DateTime<Utc>
});

model!(PublicKeys, {
    pub id: String,
    pub public_key: Vec<u8>,
    pub fingerprint: String,
    pub related_account: String,
    pub key_added_when: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>
});

model!(LoginAttempt, {
    pub id: String,
    pub method: String,
    pub method_type: String,
    pub code: Option<String>,
    pub flags: i64,
    pub related_account: String,
    pub login_attempt_finished: Option<DateTime<Utc>>,
    pub login_attempt_created_when: DateTime<Utc>,
});

model!(Sessions, {
    pub id: String,
    pub related_account: String,
    pub related_credential: Option<String>,
    pub flags: i64,
    pub session_key: String,
    pub session_created_when: DateTime<Utc>
});

model!(Accounts, {
    pub id: String,
    pub name: String,
    pub flags: i64,
    pub display_name: Option<String>,
    pub credential_id: String,
    pub email: String,
    pub email_verification_code: Option<String>,
    pub email_verification_started: Option<DateTime<Utc>>,
    pub email_verified_when: Option<DateTime<Utc>>,
    pub last_account_update: DateTime<Utc>,
    pub account_created_when: DateTime<Utc>,
});

impl Accounts
{
    pub fn get_reduced(&self) -> serde_json::Value
    {
        json!({
            "id": self.id,
            "email": self.email,
            "display_name": self.display_name,
            "flags": self.flags
        })
    }
}

model!(Modules, {
    pub name: String,
    pub run_as: String,
    pub executable: String,
    pub flags: i64,
    pub manifest_hash: Vec<u8>,
    pub binary_hash: Vec<u8>,
});

model!(Repository, {
    pub id: String,
    pub name: String,
    pub belongs_to: String,
    pub belongs_type: String,
    pub default_branch: String,
    pub flags: i64,
    pub original_creator: String,
    pub repo_created_when: DateTime<Utc>
});

model!(RepositoryRefs, {
    pub id: String,
    pub repository_id: String,
    pub ref_name: String,
    pub ref_type: String,
    pub target_oid: String,
    pub updated_when: DateTime<Utc>,
    pub updated_by: Option<String>
});

model!(RepositoryHead, {
    pub repository_id: String,
    pub symbolic_ref: String,
    pub repo_head_updated_when: DateTime<Utc>
});

// Updates:
// 1. Save the ref OID that the change was created from, it will most likely
//    update before it gets merged. Saves time later trying to query Literally
//    just the target_oid of the repository_ref currently being pointed at.
// 2. Save the id of the first patch set. This can be done inlieu of the #1, but
//    I think #1 is better.

model!(Changes, {
    pub id: String,
    pub num: i64,
    pub repository_id: String,
    pub target_ref: String,
    pub latest_patchset: String,
    pub flags: i64,
    pub patch_id: Option<String>,
    pub title: String,
    pub tree_id: String,
    pub original_account: String,
    pub changes_updated_when: DateTime<Utc>,
    pub change_created_when: DateTime<Utc>
});

model!(Patchset, {
    pub id: String,
    pub num: i64,
    pub change_id: String,
    pub parent_commit_oid: Option<String>,
    pub commit_oid: String,
    pub pushed_by: String,
    pub commit_message: String,
    pub summary: String,
    pub patchset_created_when: DateTime<Utc>
});

model!(PushRule, {
    pub id: String,
    pub repository_id: String,
    pub branch: Option<String>,
    pub protected: bool,
    pub force_push: bool
});

model!(PushRuleEntity, {
    pub rule_id: String,
    pub entity: String,
    pub exclusive: bool
});

model!(MergePolicy, {
    pub id: String,
    pub repository_id: String,
    pub branch: Option<String>,
    pub review_required: bool
});

model!(MergeRequirement, {
    pub id: String,
    pub name: String,
    pub policy: String,
    pub required_for_merge: bool,
    pub composite: bool,
    pub value: Option<i32>
});

model!(MergeRule, {
    pub id: String,
    pub requirement: String,
    pub lower: i32,
    pub upper: i32,
    pub exclusive: bool,
    pub allow_all: bool
});

model!(MergeRuleEntity, {
    pub rule_id: String,
    pub entity: String
});

model!(ChangeRequirementRulesInstance, {
    pub requirement_id: String,
    pub change_id: String,
    pub set_value: i32,
    pub set_by: String,
    pub set_on: DateTime<Utc>
});

model!(MergePolicyDynamicRequirement, {
    pub id: String,
    pub policy: String,
    pub name: String,
    pub deviation_bound_high: Option<i32>,
    pub deviation_bound_low: Option<i32>,
    pub generation: Option<i64>,
    pub created_by: String,
    pub created_on: DateTime<Utc>
});

model!(MergePolicyDynamicRequirementApply, {
    pub dynamic_requirement_id: String,
    pub requirement_id: String
});

model!(DefaultReviewer, {
    pub id: String,
    pub policy: String,
    pub reviewer: String,
    pub required: bool
});

model!(MergeQueue, {
    pub id: String,
    pub place_in_queue: i32,
    pub priority: i32,
    pub flags: i64,
    pub repository_id: String,
    pub r#ref: String,
    pub merge_request: String,
    pub latest_patchset_when_queued: String,
    pub first_patchset: String,
    pub queued_by: String,
    pub queued_on: DateTime<Utc>
});

model!(MrAssignedReviewer, {
    pub id: String,
    pub change_id: String,
    pub reviewer: String,
    pub required: bool
});

pub mod flags
{
    use bitflags::bitflags;

    bitflags! {
        #[repr(C)]
        #[derive(Debug, PartialEq, Eq)]
        pub struct People: i64 {}

        #[repr(C)]
        #[derive(Debug, PartialEq, Eq)]
        pub struct LoginAttempt: i64 {
            const FINSIHED = 1;
        }

        #[repr(C)]
        #[derive(Debug, PartialEq, Eq)]
        pub struct Change: i64 {
            const MERGED_IN = 1;
            const ABANDONED = 1 << 1;
        }

        #[repr(C)]
        #[derive(Debug, PartialEq, Eq)]
        pub struct Session: i64 {
            const INITIAL_SETUP_SESSION = 1;
            const LOCKED = 1 << 1;
            const FINSIHED = 1 << 2;
        }
        #[repr(C)]
        #[derive(Debug, PartialEq, Eq)]
        pub struct Modules: i64 {
            const CORE_MODULE = 1;
            const EXTERNAL_MODULE = 1 << 1;
        }
    }
}
