import <script context="module" lang="ts">
import gql from 'graphql-tag';
import * as UrqlSvelte from '@urql/svelte';
export type Maybe<T> = T | null;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type Omit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
  /** i54: 54-bit signed integer abstraction; represented as `i54`/`i64` in Rust, `Float` in GraphQL, `number` in TypeScript. */
  i54: number;
};

export type Card = {
  __typename?: 'Card';
  rowid?: Maybe<Scalars['i54']>;
  id?: Maybe<Scalars['Int']>;
  filesize?: Maybe<Scalars['i54']>;
  name?: Maybe<Scalars['String']>;
  content?: Maybe<Scalars['String']>;
};

export type Mutation = {
  __typename?: 'Mutation';
  addCard: Card;
  updateCard: Card;
  deleteCard: Scalars['Boolean'];
};


export type MutationAddCardArgs = {
  content: Scalars['String'];
};


export type MutationUpdateCardArgs = {
  rowid: Scalars['i54'];
  content: Scalars['String'];
};


export type MutationDeleteCardArgs = {
  rowid: Scalars['i54'];
};

export type ShortCard = {
  __typename?: 'ShortCard';
  rowid: Scalars['i54'];
  name: Scalars['String'];
};

export type Subscription = {
  __typename?: 'Subscription';
  cardStream: Card;
};

export type Query = {
  __typename?: 'Query';
  listCardsShort: Array<ShortCard>;
  listCardsFull: Array<Card>;
};


export type AddCardMutationVariables = Exact<{
  content: Scalars['String'];
}>;


export type AddCardMutation = (
  { __typename?: 'Mutation' }
  & { addCard: (
    { __typename?: 'Card' }
    & Pick<Card, 'rowid' | 'id' | 'filesize' | 'name' | 'content'>
  ) }
);

export type UpdateCardMutationVariables = Exact<{
  rowid: Scalars['i54'];
  content: Scalars['String'];
}>;


export type UpdateCardMutation = (
  { __typename?: 'Mutation' }
  & { updateCard: (
    { __typename?: 'Card' }
    & Pick<Card, 'rowid' | 'id' | 'filesize' | 'name' | 'content'>
  ) }
);

export type DeleteCardMutationVariables = Exact<{
  rowid: Scalars['i54'];
}>;


export type DeleteCardMutation = (
  { __typename?: 'Mutation' }
  & Pick<Mutation, 'deleteCard'>
);

export type CardStreamSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type CardStreamSubscription = (
  { __typename?: 'Subscription' }
  & { cardStream: (
    { __typename?: 'Card' }
    & Pick<Card, 'rowid' | 'id' | 'filesize' | 'name' | 'content'>
  ) }
);

export type ListCardsShortQueryVariables = Exact<{ [key: string]: never; }>;


export type ListCardsShortQuery = (
  { __typename?: 'Query' }
  & { listCardsShort: Array<(
    { __typename?: 'ShortCard' }
    & Pick<ShortCard, 'rowid' | 'name'>
  )> }
);

export type ListCardsFullQueryVariables = Exact<{ [key: string]: never; }>;


export type ListCardsFullQuery = (
  { __typename?: 'Query' }
  & { listCardsFull: Array<(
    { __typename?: 'Card' }
    & Pick<Card, 'rowid' | 'id' | 'filesize' | 'name' | 'content'>
  )> }
);


export const AddCardDocument = gql`
    mutation addCard($content: String!) {
  addCard(content: $content) {
    rowid
    id
    filesize
    name
    content
  }
}
    `;

export function addCardMutation() {
  return UrqlSvelte.mutation(UrqlSvelte.operationStore(AddCardDocument));
};
export const UpdateCardDocument = gql`
    mutation updateCard($rowid: i54!, $content: String!) {
  updateCard(rowid: $rowid, content: $content) {
    rowid
    id
    filesize
    name
    content
  }
}
    `;

export function updateCardMutation() {
  return UrqlSvelte.mutation(UrqlSvelte.operationStore(UpdateCardDocument));
};
export const DeleteCardDocument = gql`
    mutation deleteCard($rowid: i54!) {
  deleteCard(rowid: $rowid)
}
    `;

export function deleteCardMutation() {
  return UrqlSvelte.mutation(UrqlSvelte.operationStore(DeleteCardDocument));
};
export const CardStreamDocument = gql`
    subscription cardStream {
  cardStream {
    rowid
    id
    filesize
    name
    content
  }
}
    `;

export function cardStreamSubscription(handler) {
  return UrqlSvelte.subscription(UrqlSvelte.operationStore(CardStreamDocument), handler);
};
export const ListCardsShortDocument = gql`
    query listCardsShort {
  listCardsShort {
    rowid
    name
  }
}
    `;

export function listCardsShortQuery() {
  return UrqlSvelte.query(UrqlSvelte.operationStore(ListCardsShortDocument));
};
export const ListCardsFullDocument = gql`
    query listCardsFull {
  listCardsFull {
    rowid
    id
    filesize
    name
    content
  }
}
    `;

export function listCardsFullQuery() {
  return UrqlSvelte.query(UrqlSvelte.operationStore(ListCardsFullDocument));
};
</script>