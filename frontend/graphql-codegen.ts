import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
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
  content?: Maybe<Scalars['String']>;
  answer?: Maybe<Scalars['String']>;
  componentId?: Maybe<Scalars['String']>;
  instantiationId?: Maybe<Scalars['String']>;
  createdTime?: Maybe<Scalars['i54']>;
  modifiedTime?: Maybe<Scalars['i54']>;
  lastDisplayTime?: Maybe<Scalars['i54']>;
  nextDisplayTime?: Maybe<Scalars['i54']>;
};

export type Mutation = {
  __typename?: 'Mutation';
  addBlankCard: Card;
  recvSteps: Scalars['String'];
  deleteCard: Scalars['Boolean'];
  shuffleCards: Scalars['Boolean'];
};


export type MutationRecvStepsArgs = {
  instantiationId: Scalars['String'];
  steps: Scalars['String'];
};


export type MutationDeleteCardArgs = {
  rowid: Scalars['i54'];
};

export type ShortCard = {
  __typename?: 'ShortCard';
  rowid: Scalars['i54'];
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


export type AddBlankCardMutationVariables = Exact<{ [key: string]: never; }>;


export type AddBlankCardMutation = (
  { __typename?: 'Mutation' }
  & { addBlankCard: (
    { __typename?: 'Card' }
    & Pick<Card, 'rowid' | 'content' | 'answer' | 'componentId' | 'instantiationId' | 'createdTime' | 'modifiedTime' | 'lastDisplayTime' | 'nextDisplayTime'>
  ) }
);

export type RecvStepsMutationVariables = Exact<{
  instantiationId: Scalars['String'];
  steps: Scalars['String'];
}>;


export type RecvStepsMutation = (
  { __typename?: 'Mutation' }
  & Pick<Mutation, 'recvSteps'>
);

export type DeleteCardMutationVariables = Exact<{
  rowid: Scalars['i54'];
}>;


export type DeleteCardMutation = (
  { __typename?: 'Mutation' }
  & Pick<Mutation, 'deleteCard'>
);

export type ShuffleCardsMutationVariables = Exact<{ [key: string]: never; }>;


export type ShuffleCardsMutation = (
  { __typename?: 'Mutation' }
  & Pick<Mutation, 'shuffleCards'>
);

export type CardStreamSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type CardStreamSubscription = (
  { __typename?: 'Subscription' }
  & { cardStream: (
    { __typename?: 'Card' }
    & Pick<Card, 'rowid' | 'content' | 'answer' | 'componentId' | 'instantiationId' | 'createdTime' | 'modifiedTime' | 'lastDisplayTime' | 'nextDisplayTime'>
  ) }
);

export type ListCardsShortQueryVariables = Exact<{ [key: string]: never; }>;


export type ListCardsShortQuery = (
  { __typename?: 'Query' }
  & { listCardsShort: Array<(
    { __typename?: 'ShortCard' }
    & Pick<ShortCard, 'rowid'>
  )> }
);

export type ListCardsFullQueryVariables = Exact<{ [key: string]: never; }>;


export type ListCardsFullQuery = (
  { __typename?: 'Query' }
  & { listCardsFull: Array<(
    { __typename?: 'Card' }
    & Pick<Card, 'rowid' | 'content' | 'answer' | 'componentId' | 'instantiationId' | 'createdTime' | 'modifiedTime' | 'lastDisplayTime' | 'nextDisplayTime'>
  )> }
);


export const AddBlankCardDocument: DocumentNode<AddBlankCardMutation, AddBlankCardMutationVariables> = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"addBlankCard"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"addBlankCard"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"rowid"}},{"kind":"Field","name":{"kind":"Name","value":"content"}},{"kind":"Field","name":{"kind":"Name","value":"answer"}},{"kind":"Field","name":{"kind":"Name","value":"componentId"}},{"kind":"Field","name":{"kind":"Name","value":"instantiationId"}},{"kind":"Field","name":{"kind":"Name","value":"createdTime"}},{"kind":"Field","name":{"kind":"Name","value":"modifiedTime"}},{"kind":"Field","name":{"kind":"Name","value":"lastDisplayTime"}},{"kind":"Field","name":{"kind":"Name","value":"nextDisplayTime"}}]}}]}}]};
export const RecvStepsDocument: DocumentNode<RecvStepsMutation, RecvStepsMutationVariables> = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"recvSteps"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"instantiationId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"steps"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"recvSteps"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"instantiationId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"instantiationId"}}},{"kind":"Argument","name":{"kind":"Name","value":"steps"},"value":{"kind":"Variable","name":{"kind":"Name","value":"steps"}}}]}]}}]};
export const DeleteCardDocument: DocumentNode<DeleteCardMutation, DeleteCardMutationVariables> = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"deleteCard"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"rowid"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"i54"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"deleteCard"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"rowid"},"value":{"kind":"Variable","name":{"kind":"Name","value":"rowid"}}}]}]}}]};
export const ShuffleCardsDocument: DocumentNode<ShuffleCardsMutation, ShuffleCardsMutationVariables> = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"shuffleCards"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"shuffleCards"}}]}}]};
export const CardStreamDocument: DocumentNode<CardStreamSubscription, CardStreamSubscriptionVariables> = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"subscription","name":{"kind":"Name","value":"cardStream"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"cardStream"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"rowid"}},{"kind":"Field","name":{"kind":"Name","value":"content"}},{"kind":"Field","name":{"kind":"Name","value":"answer"}},{"kind":"Field","name":{"kind":"Name","value":"componentId"}},{"kind":"Field","name":{"kind":"Name","value":"instantiationId"}},{"kind":"Field","name":{"kind":"Name","value":"createdTime"}},{"kind":"Field","name":{"kind":"Name","value":"modifiedTime"}},{"kind":"Field","name":{"kind":"Name","value":"lastDisplayTime"}},{"kind":"Field","name":{"kind":"Name","value":"nextDisplayTime"}}]}}]}}]};
export const ListCardsShortDocument: DocumentNode<ListCardsShortQuery, ListCardsShortQueryVariables> = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"listCardsShort"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"listCardsShort"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"rowid"}}]}}]}}]};
export const ListCardsFullDocument: DocumentNode<ListCardsFullQuery, ListCardsFullQueryVariables> = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"listCardsFull"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"listCardsFull"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"rowid"}},{"kind":"Field","name":{"kind":"Name","value":"content"}},{"kind":"Field","name":{"kind":"Name","value":"answer"}},{"kind":"Field","name":{"kind":"Name","value":"componentId"}},{"kind":"Field","name":{"kind":"Name","value":"instantiationId"}},{"kind":"Field","name":{"kind":"Name","value":"createdTime"}},{"kind":"Field","name":{"kind":"Name","value":"modifiedTime"}},{"kind":"Field","name":{"kind":"Name","value":"lastDisplayTime"}},{"kind":"Field","name":{"kind":"Name","value":"nextDisplayTime"}}]}}]}}]};