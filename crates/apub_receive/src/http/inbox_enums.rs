use crate::activities::{
  comment::{
    create::CreateComment,
    delete::DeleteComment,
    dislike::DislikeComment,
    like::LikeComment,
    remove::RemoveComment,
    undo_delete::UndoDeleteComment,
    undo_dislike::UndoDislikeComment,
    undo_like::UndoLikeComment,
    undo_remove::UndoRemoveComment,
    update::UpdateComment,
  },
  community::{
    add_mod::AddMod,
    announce::AnnounceActivity,
    block_user::BlockUserFromCommunity,
    delete::DeleteCommunity,
    remove::RemoveCommunity,
    remove_mod::RemoveMod,
    undo_block_user::UndoBlockUserFromCommunity,
    undo_delete::UndoDeleteCommunity,
    undo_remove::UndoRemoveCommunity,
    update::UpdateCommunity,
  },
  following::{accept::AcceptFollowCommunity, follow::FollowCommunity, undo::UndoFollowCommunity},
  post::{
    create::CreatePost,
    delete::DeletePost,
    dislike::DislikePost,
    like::LikePost,
    remove::RemovePost,
    undo_delete::UndoDeletePost,
    undo_dislike::UndoDislikePost,
    undo_like::UndoLikePost,
    undo_remove::UndoRemovePost,
    update::UpdatePost,
  },
  private_message::{
    create::CreatePrivateMessage,
    delete::DeletePrivateMessage,
    undo_delete::UndoDeletePrivateMessage,
    update::UpdatePrivateMessage,
  },
};
use lemmy_apub_lib::ActivityHandler;
use lemmy_utils::LemmyError;
use lemmy_websocket::LemmyContext;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PersonInboxActivities {
  AcceptFollowCommunity(AcceptFollowCommunity),
  CreatePrivateMessage(CreatePrivateMessage),
  UpdatePrivateMessage(UpdatePrivateMessage),
  DeletePrivateMessage(DeletePrivateMessage),
  UndoDeletePrivateMessage(UndoDeletePrivateMessage),
  AnnounceActivity(Box<AnnounceActivity>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GroupInboxActivities {
  FollowCommunity(FollowCommunity),
  UndoFollowCommunity(UndoFollowCommunity),
  CreateComment(CreateComment),
  UpdateComment(UpdateComment),
  LikeComment(LikeComment),
  DislikeComment(DislikeComment),
  UndoLikeComment(UndoLikeComment),
  UndoDislikeComment(UndoDislikeComment),
  DeleteComment(DeleteComment),
  UndoDeleteComment(UndoDeleteComment),
  RemoveComment(RemoveComment),
  UndoRemoveComment(UndoRemoveComment),
  CreatePost(CreatePost),
  UpdatePost(UpdatePost),
  LikePost(LikePost),
  DislikePost(DislikePost),
  DeletePost(DeletePost),
  UndoDeletePost(UndoDeletePost),
  RemovePost(RemovePost),
  UndoRemovePost(UndoRemovePost),
  UndoLikePost(UndoLikePost),
  UndoDislikePost(UndoDislikePost),
  UpdateCommunity(Box<UpdateCommunity>),
  DeleteCommunity(DeleteCommunity),
  RemoveCommunity(RemoveCommunity),
  UndoDeleteCommunity(UndoDeleteCommunity),
  UndoRemoveCommunity(UndoRemoveCommunity),
  BlockUserFromCommunity(BlockUserFromCommunity),
  UndoBlockUserFromCommunity(UndoBlockUserFromCommunity),
  AddMod(AddMod),
  RemoveMod(RemoveMod),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SharedInboxActivities {
  // received by person
  AcceptFollowCommunity(AcceptFollowCommunity),
  CreatePrivateMessage(CreatePrivateMessage),
  UpdatePrivateMessage(UpdatePrivateMessage),
  DeletePrivateMessage(DeletePrivateMessage),
  UndoDeletePrivateMessage(UndoDeletePrivateMessage),
  AnnounceActivity(Box<AnnounceActivity>),
  // received by group
  FollowCommunity(FollowCommunity),
  UndoFollowCommunity(UndoFollowCommunity),
  CreateComment(CreateComment),
  UpdateComment(UpdateComment),
  LikeComment(LikeComment),
  DislikeComment(DislikeComment),
  UndoLikeComment(UndoLikeComment),
  UndoDislikeComment(UndoDislikeComment),
  DeleteComment(DeleteComment),
  UndoDeleteComment(UndoDeleteComment),
  RemoveComment(RemoveComment),
  UndoRemoveComment(UndoRemoveComment),
  CreatePost(CreatePost),
  UpdatePost(UpdatePost),
  LikePost(LikePost),
  DislikePost(DislikePost),
  DeletePost(DeletePost),
  UndoDeletePost(UndoDeletePost),
  RemovePost(RemovePost),
  UndoRemovePost(UndoRemovePost),
  UndoLikePost(UndoLikePost),
  UndoDislikePost(UndoDislikePost),
  UpdateCommunity(Box<UpdateCommunity>),
  DeleteCommunity(DeleteCommunity),
  RemoveCommunity(RemoveCommunity),
  UndoDeleteCommunity(UndoDeleteCommunity),
  UndoRemoveCommunity(UndoRemoveCommunity),
  BlockUserFromCommunity(BlockUserFromCommunity),
  UndoBlockUserFromCommunity(UndoBlockUserFromCommunity),
  AddMod(AddMod),
  RemoveMod(RemoveMod),
}

#[async_trait::async_trait(?Send)]
impl ActivityHandler for SharedInboxActivities {
  type Actor = lemmy_apub::fetcher::Actor;

  async fn verify(&self, context: &LemmyContext) -> Result<(), LemmyError> {
    self.verify(context).await
  }

  async fn receive(
    &self,
    actor: Self::Actor,
    context: &LemmyContext,
    request_counter: &mut i32,
  ) -> Result<(), LemmyError> {
    self.receive(actor, context, request_counter).await
  }
}

#[async_trait::async_trait(?Send)]
impl ActivityHandler for PersonInboxActivities {
  type Actor = lemmy_apub::fetcher::Actor;

  async fn verify(&self, context: &LemmyContext) -> Result<(), LemmyError> {
    self.verify(context).await
  }

  async fn receive(
    &self,
    actor: Self::Actor,
    context: &LemmyContext,
    request_counter: &mut i32,
  ) -> Result<(), LemmyError> {
    self.receive(actor, context, request_counter).await
  }
}

#[async_trait::async_trait(?Send)]
impl ActivityHandler for GroupInboxActivities {
  type Actor = lemmy_apub::fetcher::Actor;

  async fn verify(&self, context: &LemmyContext) -> Result<(), LemmyError> {
    self.verify(context).await
  }

  async fn receive(
    &self,
    actor: Self::Actor,
    context: &LemmyContext,
    request_counter: &mut i32,
  ) -> Result<(), LemmyError> {
    self.receive(actor, context, request_counter).await
  }
}
