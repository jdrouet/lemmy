use crate::activities::{
  post::{like::LikePost, undo_like_or_dislike_post},
  LemmyActivity,
};
use activitystreams::activity::kind::UndoType;
use lemmy_apub::check_is_apub_id_valid;
use lemmy_apub_lib::{verify_domains_match, ActivityHandler, PublicUrl};
use lemmy_db_schema::source::person::Person;
use lemmy_utils::LemmyError;
use lemmy_websocket::LemmyContext;
use url::Url;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoLikePost {
  to: PublicUrl,
  object: LemmyActivity<LikePost>,
  cc: [Url; 1],
  #[serde(rename = "type")]
  kind: UndoType,
}
#[async_trait::async_trait(?Send)]
impl ActivityHandler for LemmyActivity<UndoLikePost> {
  type Actor = Person;

  async fn verify(&self, context: &LemmyContext) -> Result<(), LemmyError> {
    verify_domains_match(&self.actor, self.id_unchecked())?;
    verify_domains_match(&self.actor, &self.inner.object.inner.object)?;
    check_is_apub_id_valid(&self.actor, false)
    //self.inner.object.verify(context).await
  }

  async fn receive(
    &self,
    actor: Self::Actor,
    context: &LemmyContext,
    request_counter: &mut i32,
  ) -> Result<(), LemmyError> {
    undo_like_or_dislike_post(
      &actor,
      &self.inner.object.inner.object,
      context,
      request_counter,
    )
    .await
  }
}
