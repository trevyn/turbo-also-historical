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

export type Mutation = {
  __typename?: 'Mutation';
  addPdf: Pdf;
  deletePdf: MutationResult;
};


export type MutationAddPdfArgs = {
  content: Scalars['String'];
};


export type MutationDeletePdfArgs = {
  rowid: Scalars['i54'];
};

export type MutationResult = {
  __typename?: 'MutationResult';
  success: Scalars['Boolean'];
};

export type Subscription = {
  __typename?: 'Subscription';
  usersSubscription: Pdf;
};

export type Query = {
  __typename?: 'Query';
  listPdfs: Array<ListPdfsResultItem>;
  listPdfs2: Array<ListPdfsResultItem2>;
};

export type ListPdfsResultItem2 = {
  __typename?: 'ListPdfsResultItem2';
  rowid: Scalars['i54'];
  name: Scalars['String'];
};

export type ListPdfsResultItem = {
  __typename?: 'ListPdfsResultItem';
  rowid: Scalars['i54'];
  name: Scalars['String'];
};

export type Pdf = {
  __typename?: 'Pdf';
  rowid?: Maybe<Scalars['i54']>;
  id?: Maybe<Scalars['Int']>;
  filesize?: Maybe<Scalars['i54']>;
  name?: Maybe<Scalars['String']>;
  content?: Maybe<Scalars['String']>;
};


export type AddPdfMutationVariables = Exact<{
  content: Scalars['String'];
}>;


export type AddPdfMutation = (
  { __typename?: 'Mutation' }
  & { addPdf: (
    { __typename?: 'Pdf' }
    & Pick<Pdf, 'rowid' | 'id' | 'filesize' | 'name' | 'content'>
  ) }
);

export type DeletePdfMutationVariables = Exact<{
  rowid: Scalars['i54'];
}>;


export type DeletePdfMutation = (
  { __typename?: 'Mutation' }
  & { deletePdf: (
    { __typename?: 'MutationResult' }
    & Pick<MutationResult, 'success'>
  ) }
);

export type UsersSubscriptionSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type UsersSubscriptionSubscription = (
  { __typename?: 'Subscription' }
  & { usersSubscription: (
    { __typename?: 'Pdf' }
    & Pick<Pdf, 'rowid' | 'id' | 'filesize' | 'name' | 'content'>
  ) }
);

export type ListPdfsQueryVariables = Exact<{ [key: string]: never; }>;


export type ListPdfsQuery = (
  { __typename?: 'Query' }
  & { listPdfs: Array<(
    { __typename?: 'ListPdfsResultItem' }
    & Pick<ListPdfsResultItem, 'rowid' | 'name'>
  )> }
);

export type ListPdfs2QueryVariables = Exact<{ [key: string]: never; }>;


export type ListPdfs2Query = (
  { __typename?: 'Query' }
  & { listPdfs2: Array<(
    { __typename?: 'ListPdfsResultItem2' }
    & Pick<ListPdfsResultItem2, 'rowid' | 'name'>
  )> }
);


export const AddPdfDocument = gql`
    mutation addPdf($content: String!) {
  addPdf(content: $content) {
    rowid
    id
    filesize
    name
    content
  }
}
    `;

export function addPdfMutation() {
  return UrqlSvelte.mutation(UrqlSvelte.operationStore(AddPdfDocument));
};
export const DeletePdfDocument = gql`
    mutation deletePdf($rowid: i54!) {
  deletePdf(rowid: $rowid) {
    success
  }
}
    `;

export function deletePdfMutation() {
  return UrqlSvelte.mutation(UrqlSvelte.operationStore(DeletePdfDocument));
};
export const UsersSubscriptionDocument = gql`
    subscription usersSubscription {
  usersSubscription {
    rowid
    id
    filesize
    name
    content
  }
}
    `;

export function usersSubscriptionSubscription(handler) {
  return UrqlSvelte.subscription(UrqlSvelte.operationStore(UsersSubscriptionDocument), handler);
};
export const ListPdfsDocument = gql`
    query listPdfs {
  listPdfs {
    rowid
    name
  }
}
    `;

export function listPdfsQuery() {
  return UrqlSvelte.query(UrqlSvelte.operationStore(ListPdfsDocument));
};
export const ListPdfs2Document = gql`
    query listPdfs2 {
  listPdfs2 {
    rowid
    name
  }
}
    `;

export function listPdfs2Query() {
  return UrqlSvelte.query(UrqlSvelte.operationStore(ListPdfs2Document));
};
</script>